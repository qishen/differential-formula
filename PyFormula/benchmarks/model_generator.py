import random
from datetime import datetime
import os
import os.path
import subprocess

from utils import utils


class GraphGenerator:
    def __init__(self):
        self.node_names = []
        self.edge_names = {}

    def generate_duplicate_free_string(self, names, str_size):
        random_str = utils.random_string(str_size)
        while random_str in names:
            random_str = utils.random_string(str_size)
        return random_str

    def create_nodes(self, num):
        for i in range(num):
            random_str = self.generate_duplicate_free_string(self.node_names, 8)
            self.node_names.append(random_str)
        nodes_str = ''
        for i in range(num):
            nodes_str += 'node_%s is Node(\"%s\").\n' % (self.node_names[i], self.node_names[i])
        return nodes_str

    def create_edges(self, num):
        if num > len(self.node_names) * len(self.node_names):
            raise Exception('Exceed maximum number of edges that can be generated.')
        length = len(self.node_names)
        for i in range(num):
            src = self.node_names[random.randint(0, length-1)]
            dst = self.node_names[random.randint(0, length-1)]
            while src in self.edge_names and dst in self.edge_names[src]:
                src = self.node_names[random.randint(0, length-1)]
                dst = self.node_names[random.randint(0, length-1)]
            if src in self.edge_names:
                self.edge_names[src].append(dst)
            else:
                self.edge_names[src] = [dst]

        edges_str = ''
        for src in self.edge_names:
            dst_list = self.edge_names[src]
            for dst in dst_list:
                edge_str = 'Edge(%s, %s).\n' % ('node_' + src, 'node_' + dst)
                edges_str += edge_str
        return edges_str

    def instantiate_graph_template(self, node_num, edge_num, domain):
        self.node_names.clear()
        self.edge_names.clear()

        nodes_str = self.create_nodes(node_num)
        edges_str = self.create_edges(edge_num)

        f = open('templates/graphs.4ml', 'r+')
        template = f.read()
        f.close()

        generated_filename = '%s_model_%sN_%sE.4ml' % (domain, str(node_num), str(edge_num))
        f = open('generated/' + generated_filename, 'w+')
        split_str = '//' + domain.lower()
        texts = template.split(split_str, 1)
        program = texts[0] + nodes_str + edges_str + texts[1]
        f.write(program)
        f.close()

        return program


class HyperGraphGenerator(GraphGenerator):
    def __init__(self):
        super().__init__()
        self.hypernode_names_list = []
        self.hyperedge_names = {}

    def create_hypernodes(self, prev_node_names, num, cluster_size):
        """
        Randomly cluster some nodes to create a supernode.
        :param prev_node_names:
        :param num:
        :param cluster_size:
        :return:
        """
        hypernode_names = []
        hypernodes_str = ''
        for i in range(num):
            nodeset_str = 'NIL'
            for j in range(cluster_size):
                node_str = 'hypernode_%s' % prev_node_names[random.randint(0, len(prev_node_names)-1)]
                nodeset_str = 'NodeSet(%s, %s)' % (node_str, nodeset_str)

            random_str = self.generate_duplicate_free_string(hypernode_names, 8)
            hypernode_names.append(random_str)
            hypernode_str = 'hypernode_%s is HyperNode(%s).\n' % (random_str, nodeset_str)
            hypernodes_str += hypernode_str
        return hypernode_names, hypernodes_str

    def create_nested_hypernodes(self, layer_num, layers, cluster_size):
        """
        Wrap each node as hypernode and build new hypernodes upon existing hypernodes.
        Each layer has the same number of supernodes and depends only on previous layer
        of supernodes instead of all supernodes.
        :param layer_num:
        :param layers:
        :param cluster_size:
        :return:
        """
        final_hypernodes_str = ''
        initial_nodes_str = self.create_nodes(layer_num)
        initial_hypernode_names = []
        final_hypernodes_str += initial_nodes_str

        for i in range(layer_num):
            random_str = self.generate_duplicate_free_string(initial_hypernode_names, 8)
            initial_hypernode_names.append(random_str)
            hypernode_str = 'hypernode_%s is HyperNode(node_%s).\n' % (random_str, self.node_names[i])
            final_hypernodes_str += hypernode_str
        self.hypernode_names_list.append(initial_hypernode_names)

        for i in range(layers-1):
            intermediate_hypernode_names, intermediate_hypernodes_str = \
                self.create_hypernodes(self.hypernode_names_list[-1], layer_num, cluster_size)
            final_hypernodes_str += intermediate_hypernodes_str
            self.hypernode_names_list.append(intermediate_hypernode_names)
        return final_hypernodes_str

    def create_hyperedges(self, num):
        """
        Randomly generate edges on all existing supernodes.
        :param num:
        :return:
        """
        all_hypernodes = []
        for l in self.hypernode_names_list:
            all_hypernodes += l

        length = 0
        for hypernode_names in self.hypernode_names_list:
            length += len(hypernode_names)

        if num > length * length:
            raise Exception('Exceed maximum number of edges that can be generated.')

        for i in range(num):
            src = all_hypernodes[random.randint(0, length-1)]
            dst = all_hypernodes[random.randint(0, length-1)]
            while src in self.hyperedge_names and dst in self.hyperedge_names[src]:
                src = all_hypernodes[random.randint(0, length-1)]
                dst = all_hypernodes[random.randint(0, length-1)]
            if src in self.hyperedge_names:
                self.hyperedge_names[src].append(dst)
            else:
                self.hyperedge_names[src] = [dst]

        edges_str = ''
        for src in self.hyperedge_names:
            dst_list = self.hyperedge_names[src]
            for dst in dst_list:
                edge_str = 'HyperEdge(%s, %s).\n' % ('hypernode_' + src, 'hypernode_' + dst)
                edges_str += edge_str
        return edges_str

    def instantiate_hypergraph_template(self, layer_num, layers, cluster_size, edge_num, domain):
        self.hypernode_names_list.clear()
        self.hyperedge_names.clear()

        hypernodes_str = self.create_nested_hypernodes(layer_num, layers, cluster_size)
        hyperedges_str = self.create_hyperedges(edge_num)

        f = open('templates/graphs.4ml', 'r+')
        template = f.read()
        f.close()

        generated_filename = '%s_model_%sNx%sL_%sC_%sE.4ml' % (domain, str(layer_num), str(layers),
                                                                      str(cluster_size), str(edge_num))
        f = open('generated/' + generated_filename, 'w+')
        split_str = '//' + domain.lower()
        texts = template.split(split_str, 1)
        program = texts[0] + hypernodes_str + hyperedges_str + texts[1]
        f.write(program)
        f.close()

        return program


if __name__ == '__main__':
    generator = GraphGenerator()
    node_num = 50
    for i in range(8):
        edge_num = (node_num * node_num) // 100
        generator.instantiate_graph_template(node_num, edge_num, 'GraphNonrecur')
        node_num = node_num * 2

    program = generator.instantiate_graph_template(10000, 2000, 'GraphNonrecur')
    print(program)

    hyper_generator = HyperGraphGenerator()
    program = hyper_generator.instantiate_hypergraph_template(1000, 8, 5, 200, 'HGraphNonrecur')
    print(program)

'''
    cmd = os.path.join(os.path.abspath('.'), 'executable/CommandLine.exe')
    print(cmd)
    #os.system(cmd)

    p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stdin=subprocess.PIPE, shell=True)
    #print(p.stdout.read().decode())
    #p.stdin.write('ls\n'.encode())
    (output, err) = p.communicate(input='ls'.encode())
    print(p.stdout.read().decode())

    p_status = p.wait()
    print(output.decode())
    print(p_status)
'''





