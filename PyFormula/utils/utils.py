import random
import string


def random_string(length=32):
    return ''.join([random.choice(string.ascii_letters + string.digits) for n in range(length)])
