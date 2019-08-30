# Generated from C:/Users/Victor/Desktop/projects/FormulaCore/PyFormula/grammar\Formula.g4 by ANTLR 4.7.2
# encoding: utf-8
from antlr4 import *
from io import StringIO
from typing.io import TextIO
import sys


def serializedATN():
    with StringIO() as buf:
        buf.write("\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3H")
        buf.write("\u0258\4\2\t\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7")
        buf.write("\4\b\t\b\4\t\t\t\4\n\t\n\4\13\t\13\4\f\t\f\4\r\t\r\4\16")
        buf.write("\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22\4\23\t\23")
        buf.write("\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31")
        buf.write("\t\31\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36")
        buf.write("\4\37\t\37\4 \t \4!\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t")
        buf.write("&\4\'\t\'\4(\t(\4)\t)\4*\t*\4+\t+\4,\t,\4-\t-\4.\t.\4")
        buf.write("/\t/\4\60\t\60\4\61\t\61\4\62\t\62\4\63\t\63\4\64\t\64")
        buf.write("\4\65\t\65\4\66\t\66\4\67\t\67\48\t8\49\t9\4:\t:\4;\t")
        buf.write(";\4<\t<\4=\t=\4>\t>\4?\t?\4@\t@\4A\tA\4B\tB\3\2\3\2\3")
        buf.write("\2\3\2\3\3\3\3\3\3\3\3\3\4\3\4\3\4\7\4\u0090\n\4\f\4\16")
        buf.write("\4\u0093\13\4\3\5\3\5\3\5\3\5\3\6\3\6\5\6\u009b\n\6\3")
        buf.write("\6\5\6\u009e\n\6\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\b\7\b\u00a8")
        buf.write("\n\b\f\b\16\b\u00ab\13\b\3\t\3\t\3\t\3\t\5\t\u00b1\n\t")
        buf.write("\3\n\3\n\3\n\7\n\u00b6\n\n\f\n\16\n\u00b9\13\n\3\13\3")
        buf.write("\13\3\13\5\13\u00be\n\13\3\13\3\13\5\13\u00c2\n\13\3\f")
        buf.write("\3\f\3\f\3\f\3\f\7\f\u00c9\n\f\f\f\16\f\u00cc\13\f\3\f")
        buf.write("\3\f\3\r\3\r\5\r\u00d2\n\r\3\16\3\16\3\16\3\16\3\16\3")
        buf.write("\16\3\17\3\17\5\17\u00dc\n\17\3\17\3\17\3\20\3\20\3\20")
        buf.write("\7\20\u00e3\n\20\f\20\16\20\u00e6\13\20\3\21\3\21\3\21")
        buf.write("\3\21\5\21\u00ec\n\21\3\22\5\22\u00ef\n\22\3\22\3\22\3")
        buf.write("\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\5\23")
        buf.write("\u00fd\n\23\3\24\3\24\3\24\3\24\3\24\3\24\3\24\3\24\3")
        buf.write("\25\7\25\u0108\n\25\f\25\16\25\u010b\13\25\3\26\5\26\u010e")
        buf.write("\n\26\3\26\3\26\3\27\3\27\3\30\3\30\3\30\5\30\u0117\n")
        buf.write("\30\3\30\3\30\3\31\7\31\u011c\n\31\f\31\16\31\u011f\13")
        buf.write("\31\3\32\3\32\5\32\u0123\n\32\3\33\5\33\u0126\n\33\3\33")
        buf.write("\3\33\3\34\3\34\3\34\3\34\3\34\3\34\3\34\3\34\3\34\3\34")
        buf.write("\3\34\3\34\3\34\3\34\5\34\u0138\n\34\3\35\3\35\3\36\3")
        buf.write("\36\5\36\u013e\n\36\3\37\5\37\u0141\n\37\3\37\3\37\3\37")
        buf.write("\3\37\3\37\3 \3 \3 \5 \u014b\n \3!\3!\3!\7!\u0150\n!\f")
        buf.write("!\16!\u0153\13!\3!\3!\3\"\3\"\5\"\u0159\n\"\3\"\3\"\3")
        buf.write("#\3#\3#\5#\u0160\n#\3#\3#\3$\3$\5$\u0166\n$\3%\3%\3%\3")
        buf.write("%\5%\u016c\n%\3&\3&\3&\7&\u0171\n&\f&\16&\u0174\13&\3")
        buf.write("\'\5\'\u0177\n\'\3\'\3\'\3(\3(\3(\3(\5(\u017f\n(\3)\3")
        buf.write(")\3)\5)\u0184\n)\3)\3)\3)\3)\3)\3)\3)\5)\u018d\n)\3*\3")
        buf.write("*\3*\7*\u0192\n*\f*\16*\u0195\13*\3+\3+\3,\3,\3,\5,\u019c")
        buf.write("\n,\3,\7,\u019f\n,\f,\16,\u01a2\13,\3-\3-\5-\u01a6\n-")
        buf.write("\3-\5-\u01a9\n-\3-\3-\5-\u01ad\n-\3.\3.\3.\3.\3.\5.\u01b4")
        buf.write("\n.\3/\3/\3/\7/\u01b9\n/\f/\16/\u01bc\13/\3\60\3\60\3")
        buf.write("\60\3\60\3\60\5\60\u01c3\n\60\3\61\3\61\3\61\3\61\3\62")
        buf.write("\3\62\3\62\3\62\5\62\u01cd\n\62\3\62\3\62\3\63\3\63\3")
        buf.write("\63\5\63\u01d4\n\63\3\64\3\64\3\64\3\64\3\64\3\65\3\65")
        buf.write("\3\65\3\65\3\65\3\65\3\65\3\66\3\66\3\66\3\66\3\66\3\66")
        buf.write("\3\66\3\66\3\67\3\67\3\67\7\67\u01ed\n\67\f\67\16\67\u01f0")
        buf.write("\13\67\38\38\38\78\u01f5\n8\f8\168\u01f8\138\39\59\u01fb")
        buf.write("\n9\39\39\59\u01ff\n9\39\39\39\39\39\39\39\39\39\39\3")
        buf.write("9\59\u020c\n9\39\39\39\39\59\u0212\n9\3:\3:\3:\3:\3:\7")
        buf.write(":\u0219\n:\f:\16:\u021c\13:\3:\3:\3:\5:\u0221\n:\3;\3")
        buf.write(";\3;\7;\u0226\n;\f;\16;\u0229\13;\3<\3<\3<\3<\3<\3<\3")
        buf.write("<\5<\u0232\n<\5<\u0234\n<\3<\3<\3<\3<\3<\3<\3<\3<\3<\7")
        buf.write("<\u023f\n<\f<\16<\u0242\13<\3=\3=\5=\u0246\n=\3>\3>\3")
        buf.write(">\7>\u024b\n>\f>\16>\u024e\13>\3?\3?\3@\3@\3A\3A\3B\3")
        buf.write("B\3B\2\3vC\2\4\6\b\n\f\16\20\22\24\26\30\32\34\36 \"$")
        buf.write("&(*,.\60\62\64\668:<>@BDFHJLNPRTVXZ\\^`bdfhjlnprtvxz|")
        buf.write("~\u0080\u0082\2\f\3\2\31\33\3\2\7\b\4\2\17\23\25\25\4")
        buf.write("\2\r\r\60\60\3\2;<\3\29:\3\2&)\3\29=\4\2\60\60\638\3\2")
        buf.write(">?\2\u0260\2\u0084\3\2\2\2\4\u0088\3\2\2\2\6\u008c\3\2")
        buf.write("\2\2\b\u0094\3\2\2\2\n\u009d\3\2\2\2\f\u009f\3\2\2\2\16")
        buf.write("\u00a9\3\2\2\2\20\u00b0\3\2\2\2\22\u00b2\3\2\2\2\24\u00ba")
        buf.write("\3\2\2\2\26\u00c3\3\2\2\2\30\u00cf\3\2\2\2\32\u00d3\3")
        buf.write("\2\2\2\34\u00d9\3\2\2\2\36\u00df\3\2\2\2 \u00eb\3\2\2")
        buf.write("\2\"\u00ee\3\2\2\2$\u00fc\3\2\2\2&\u00fe\3\2\2\2(\u0109")
        buf.write("\3\2\2\2*\u010d\3\2\2\2,\u0111\3\2\2\2.\u0113\3\2\2\2")
        buf.write("\60\u011d\3\2\2\2\62\u0122\3\2\2\2\64\u0125\3\2\2\2\66")
        buf.write("\u0137\3\2\2\28\u0139\3\2\2\2:\u013b\3\2\2\2<\u0140\3")
        buf.write("\2\2\2>\u0147\3\2\2\2@\u014c\3\2\2\2B\u0158\3\2\2\2D\u015c")
        buf.write("\3\2\2\2F\u0163\3\2\2\2H\u0167\3\2\2\2J\u0172\3\2\2\2")
        buf.write("L\u0176\3\2\2\2N\u017e\3\2\2\2P\u018c\3\2\2\2R\u018e\3")
        buf.write("\2\2\2T\u0196\3\2\2\2V\u0198\3\2\2\2X\u01a5\3\2\2\2Z\u01b3")
        buf.write("\3\2\2\2\\\u01b5\3\2\2\2^\u01c2\3\2\2\2`\u01c4\3\2\2\2")
        buf.write("b\u01c8\3\2\2\2d\u01d3\3\2\2\2f\u01d5\3\2\2\2h\u01da\3")
        buf.write("\2\2\2j\u01e1\3\2\2\2l\u01e9\3\2\2\2n\u01f1\3\2\2\2p\u0211")
        buf.write("\3\2\2\2r\u0220\3\2\2\2t\u0222\3\2\2\2v\u0233\3\2\2\2")
        buf.write("x\u0245\3\2\2\2z\u0247\3\2\2\2|\u024f\3\2\2\2~\u0251\3")
        buf.write("\2\2\2\u0080\u0253\3\2\2\2\u0082\u0255\3\2\2\2\u0084\u0085")
        buf.write("\7B\2\2\u0085\u0086\5\6\4\2\u0086\u0087\7C\2\2\u0087\3")
        buf.write("\3\2\2\2\u0088\u0089\7B\2\2\u0089\u008a\5\6\4\2\u008a")
        buf.write("\u008b\7C\2\2\u008b\5\3\2\2\2\u008c\u0091\5\b\5\2\u008d")
        buf.write("\u008e\7\61\2\2\u008e\u0090\5\b\5\2\u008f\u008d\3\2\2")
        buf.write("\2\u0090\u0093\3\2\2\2\u0091\u008f\3\2\2\2\u0091\u0092")
        buf.write("\3\2\2\2\u0092\7\3\2\2\2\u0093\u0091\3\2\2\2\u0094\u0095")
        buf.write("\5z>\2\u0095\u0096\7\63\2\2\u0096\u0097\5|?\2\u0097\t")
        buf.write("\3\2\2\2\u0098\u009e\7\2\2\3\u0099\u009b\5\f\7\2\u009a")
        buf.write("\u0099\3\2\2\2\u009a\u009b\3\2\2\2\u009b\u009c\3\2\2\2")
        buf.write("\u009c\u009e\5\16\b\2\u009d\u0098\3\2\2\2\u009d\u009a")
        buf.write("\3\2\2\2\u009e\13\3\2\2\2\u009f\u00a0\7!\2\2\u00a0\u00a1")
        buf.write("\7%\2\2\u00a1\u00a2\7\"\2\2\u00a2\u00a3\7)\2\2\u00a3\u00a4")
        buf.write("\7#\2\2\u00a4\u00a5\7%\2\2\u00a5\r\3\2\2\2\u00a6\u00a8")
        buf.write("\5\20\t\2\u00a7\u00a6\3\2\2\2\u00a8\u00ab\3\2\2\2\u00a9")
        buf.write("\u00a7\3\2\2\2\u00a9\u00aa\3\2\2\2\u00aa\17\3\2\2\2\u00ab")
        buf.write("\u00a9\3\2\2\2\u00ac\u00b1\5D#\2\u00ad\u00b1\5.\30\2\u00ae")
        buf.write("\u00b1\5\26\f\2\u00af\u00b1\5&\24\2\u00b0\u00ac\3\2\2")
        buf.write("\2\u00b0\u00ad\3\2\2\2\u00b0\u00ae\3\2\2\2\u00b0\u00af")
        buf.write("\3\2\2\2\u00b1\21\3\2\2\2\u00b2\u00b7\5\24\13\2\u00b3")
        buf.write("\u00b4\7\61\2\2\u00b4\u00b6\5\24\13\2\u00b5\u00b3\3\2")
        buf.write("\2\2\u00b6\u00b9\3\2\2\2\u00b7\u00b5\3\2\2\2\u00b7\u00b8")
        buf.write("\3\2\2\2\u00b8\23\3\2\2\2\u00b9\u00b7\3\2\2\2\u00ba\u00bd")
        buf.write("\7%\2\2\u00bb\u00bc\7-\2\2\u00bc\u00be\7%\2\2\u00bd\u00bb")
        buf.write("\3\2\2\2\u00bd\u00be\3\2\2\2\u00be\u00c1\3\2\2\2\u00bf")
        buf.write("\u00c0\7\13\2\2\u00c0\u00c2\7)\2\2\u00c1\u00bf\3\2\2\2")
        buf.write("\u00c1\u00c2\3\2\2\2\u00c2\25\3\2\2\2\u00c3\u00c4\7\5")
        buf.write("\2\2\u00c4\u00c5\7%\2\2\u00c5\u00c6\5\30\r\2\u00c6\u00ca")
        buf.write("\7@\2\2\u00c7\u00c9\5\"\22\2\u00c8\u00c7\3\2\2\2\u00c9")
        buf.write("\u00cc\3\2\2\2\u00ca\u00c8\3\2\2\2\u00ca\u00cb\3\2\2\2")
        buf.write("\u00cb\u00cd\3\2\2\2\u00cc\u00ca\3\2\2\2\u00cd\u00ce\7")
        buf.write("A\2\2\u00ce\27\3\2\2\2\u00cf\u00d1\5\32\16\2\u00d0\u00d2")
        buf.write("\5\2\2\2\u00d1\u00d0\3\2\2\2\u00d1\u00d2\3\2\2\2\u00d2")
        buf.write("\31\3\2\2\2\u00d3\u00d4\5\34\17\2\u00d4\u00d5\7\n\2\2")
        buf.write("\u00d5\u00d6\7D\2\2\u00d6\u00d7\5\22\n\2\u00d7\u00d8\7")
        buf.write("E\2\2\u00d8\33\3\2\2\2\u00d9\u00db\7D\2\2\u00da\u00dc")
        buf.write("\5\36\20\2\u00db\u00da\3\2\2\2\u00db\u00dc\3\2\2\2\u00dc")
        buf.write("\u00dd\3\2\2\2\u00dd\u00de\7E\2\2\u00de\35\3\2\2\2\u00df")
        buf.write("\u00e4\5 \21\2\u00e0\u00e1\7\61\2\2\u00e1\u00e3\5 \21")
        buf.write("\2\u00e2\u00e0\3\2\2\2\u00e3\u00e6\3\2\2\2\u00e4\u00e2")
        buf.write("\3\2\2\2\u00e4\u00e5\3\2\2\2\u00e5\37\3\2\2\2\u00e6\u00e4")
        buf.write("\3\2\2\2\u00e7\u00e8\7%\2\2\u00e8\u00e9\7\60\2\2\u00e9")
        buf.write("\u00ec\5R*\2\u00ea\u00ec\5\24\13\2\u00eb\u00e7\3\2\2\2")
        buf.write("\u00eb\u00ea\3\2\2\2\u00ec!\3\2\2\2\u00ed\u00ef\5\4\3")
        buf.write("\2\u00ee\u00ed\3\2\2\2\u00ee\u00ef\3\2\2\2\u00ef\u00f0")
        buf.write("\3\2\2\2\u00f0\u00f1\5$\23\2\u00f1#\3\2\2\2\u00f2\u00fd")
        buf.write("\5`\61\2\u00f3\u00fd\5P)\2\u00f4\u00f5\7\26\2\2\u00f5")
        buf.write("\u00f6\5l\67\2\u00f6\u00f7\7/\2\2\u00f7\u00fd\3\2\2\2")
        buf.write("\u00f8\u00f9\7\27\2\2\u00f9\u00fa\5l\67\2\u00fa\u00fb")
        buf.write("\7/\2\2\u00fb\u00fd\3\2\2\2\u00fc\u00f2\3\2\2\2\u00fc")
        buf.write("\u00f3\3\2\2\2\u00fc\u00f4\3\2\2\2\u00fc\u00f8\3\2\2\2")
        buf.write("\u00fd%\3\2\2\2\u00fe\u00ff\7\5\2\2\u00ff\u0100\7\6\2")
        buf.write("\2\u0100\u0101\7%\2\2\u0101\u0102\5\30\r\2\u0102\u0103")
        buf.write("\7@\2\2\u0103\u0104\5(\25\2\u0104\u0105\7A\2\2\u0105\'")
        buf.write("\3\2\2\2\u0106\u0108\5*\26\2\u0107\u0106\3\2\2\2\u0108")
        buf.write("\u010b\3\2\2\2\u0109\u0107\3\2\2\2\u0109\u010a\3\2\2\2")
        buf.write("\u010a)\3\2\2\2\u010b\u0109\3\2\2\2\u010c\u010e\5\4\3")
        buf.write("\2\u010d\u010c\3\2\2\2\u010d\u010e\3\2\2\2\u010e\u010f")
        buf.write("\3\2\2\2\u010f\u0110\5,\27\2\u0110+\3\2\2\2\u0111\u0112")
        buf.write("\7%\2\2\u0112-\3\2\2\2\u0113\u0114\5:\36\2\u0114\u0116")
        buf.write("\7@\2\2\u0115\u0117\5\60\31\2\u0116\u0115\3\2\2\2\u0116")
        buf.write("\u0117\3\2\2\2\u0117\u0118\3\2\2\2\u0118\u0119\7A\2\2")
        buf.write("\u0119/\3\2\2\2\u011a\u011c\5\62\32\2\u011b\u011a\3\2")
        buf.write("\2\2\u011c\u011f\3\2\2\2\u011d\u011b\3\2\2\2\u011d\u011e")
        buf.write("\3\2\2\2\u011e\61\3\2\2\2\u011f\u011d\3\2\2\2\u0120\u0123")
        buf.write("\5@!\2\u0121\u0123\5\64\33\2\u0122\u0120\3\2\2\2\u0122")
        buf.write("\u0121\3\2\2\2\u0123\63\3\2\2\2\u0124\u0126\5\4\3\2\u0125")
        buf.write("\u0124\3\2\2\2\u0125\u0126\3\2\2\2\u0126\u0127\3\2\2\2")
        buf.write("\u0127\u0128\5\66\34\2\u0128\65\3\2\2\2\u0129\u012a\7")
        buf.write("\26\2\2\u012a\u012b\5l\67\2\u012b\u012c\7/\2\2\u012c\u0138")
        buf.write("\3\2\2\2\u012d\u012e\7\27\2\2\u012e\u012f\5l\67\2\u012f")
        buf.write("\u0130\7/\2\2\u0130\u0138\3\2\2\2\u0131\u0132\7\27\2\2")
        buf.write("\u0132\u0133\58\35\2\u0133\u0134\7&\2\2\u0134\u0135\7")
        buf.write("%\2\2\u0135\u0136\7/\2\2\u0136\u0138\3\2\2\2\u0137\u0129")
        buf.write("\3\2\2\2\u0137\u012d\3\2\2\2\u0137\u0131\3\2\2\2\u0138")
        buf.write("\67\3\2\2\2\u0139\u013a\t\2\2\2\u013a9\3\2\2\2\u013b\u013d")
        buf.write("\5> \2\u013c\u013e\5\2\2\2\u013d\u013c\3\2\2\2\u013d\u013e")
        buf.write("\3\2\2\2\u013e;\3\2\2\2\u013f\u0141\7\34\2\2\u0140\u013f")
        buf.write("\3\2\2\2\u0140\u0141\3\2\2\2\u0141\u0142\3\2\2\2\u0142")
        buf.write("\u0143\7\4\2\2\u0143\u0144\7%\2\2\u0144\u0145\7\t\2\2")
        buf.write("\u0145\u0146\5\24\13\2\u0146=\3\2\2\2\u0147\u014a\5<\37")
        buf.write("\2\u0148\u0149\t\3\2\2\u0149\u014b\5\22\n\2\u014a\u0148")
        buf.write("\3\2\2\2\u014a\u014b\3\2\2\2\u014b?\3\2\2\2\u014c\u0151")
        buf.write("\5B\"\2\u014d\u014e\7\61\2\2\u014e\u0150\5B\"\2\u014f")
        buf.write("\u014d\3\2\2\2\u0150\u0153\3\2\2\2\u0151\u014f\3\2\2\2")
        buf.write("\u0151\u0152\3\2\2\2\u0152\u0154\3\2\2\2\u0153\u0151\3")
        buf.write("\2\2\2\u0154\u0155\7/\2\2\u0155A\3\2\2\2\u0156\u0157\7")
        buf.write("%\2\2\u0157\u0159\7\r\2\2\u0158\u0156\3\2\2\2\u0158\u0159")
        buf.write("\3\2\2\2\u0159\u015a\3\2\2\2\u015a\u015b\5r:\2\u015bC")
        buf.write("\3\2\2\2\u015c\u015d\5F$\2\u015d\u015f\7@\2\2\u015e\u0160")
        buf.write("\5J&\2\u015f\u015e\3\2\2\2\u015f\u0160\3\2\2\2\u0160\u0161")
        buf.write("\3\2\2\2\u0161\u0162\7A\2\2\u0162E\3\2\2\2\u0163\u0165")
        buf.write("\5H%\2\u0164\u0166\5\2\2\2\u0165\u0164\3\2\2\2\u0165\u0166")
        buf.write("\3\2\2\2\u0166G\3\2\2\2\u0167\u0168\7\3\2\2\u0168\u016b")
        buf.write("\7%\2\2\u0169\u016a\t\3\2\2\u016a\u016c\5\22\n\2\u016b")
        buf.write("\u0169\3\2\2\2\u016b\u016c\3\2\2\2\u016cI\3\2\2\2\u016d")
        buf.write("\u016e\5L\'\2\u016e\u016f\7/\2\2\u016f\u0171\3\2\2\2\u0170")
        buf.write("\u016d\3\2\2\2\u0171\u0174\3\2\2\2\u0172\u0170\3\2\2\2")
        buf.write("\u0172\u0173\3\2\2\2\u0173K\3\2\2\2\u0174\u0172\3\2\2")
        buf.write("\2\u0175\u0177\5\4\3\2\u0176\u0175\3\2\2\2\u0176\u0177")
        buf.write("\3\2\2\2\u0177\u0178\3\2\2\2\u0178\u0179\5N(\2\u0179M")
        buf.write("\3\2\2\2\u017a\u017f\5P)\2\u017b\u017f\5`\61\2\u017c\u017d")
        buf.write("\7\30\2\2\u017d\u017f\5l\67\2\u017e\u017a\3\2\2\2\u017e")
        buf.write("\u017b\3\2\2\2\u017e\u017c\3\2\2\2\u017fO\3\2\2\2\u0180")
        buf.write("\u0181\7%\2\2\u0181\u0183\7+\2\2\u0182\u0184\5T+\2\u0183")
        buf.write("\u0182\3\2\2\2\u0183\u0184\3\2\2\2\u0184\u0185\3\2\2\2")
        buf.write("\u0185\u0186\7D\2\2\u0186\u0187\5V,\2\u0187\u0188\7E\2")
        buf.write("\2\u0188\u018d\3\2\2\2\u0189\u018a\7%\2\2\u018a\u018b")
        buf.write("\7+\2\2\u018b\u018d\5R*\2\u018c\u0180\3\2\2\2\u018c\u0189")
        buf.write("\3\2\2\2\u018dQ\3\2\2\2\u018e\u0193\5Z.\2\u018f\u0190")
        buf.write("\79\2\2\u0190\u0192\5Z.\2\u0191\u018f\3\2\2\2\u0192\u0195")
        buf.write("\3\2\2\2\u0193\u0191\3\2\2\2\u0193\u0194\3\2\2\2\u0194")
        buf.write("S\3\2\2\2\u0195\u0193\3\2\2\2\u0196\u0197\t\4\2\2\u0197")
        buf.write("U\3\2\2\2\u0198\u01a0\5X-\2\u0199\u019c\7\61\2\2\u019a")
        buf.write("\u019c\5\u0082B\2\u019b\u0199\3\2\2\2\u019b\u019a\3\2")
        buf.write("\2\2\u019c\u019d\3\2\2\2\u019d\u019f\5X-\2\u019e\u019b")
        buf.write("\3\2\2\2\u019f\u01a2\3\2\2\2\u01a0\u019e\3\2\2\2\u01a0")
        buf.write("\u01a1\3\2\2\2\u01a1W\3\2\2\2\u01a2\u01a0\3\2\2\2\u01a3")
        buf.write("\u01a4\7%\2\2\u01a4\u01a6\7\60\2\2\u01a5\u01a3\3\2\2\2")
        buf.write("\u01a5\u01a6\3\2\2\2\u01a6\u01a8\3\2\2\2\u01a7\u01a9\7")
        buf.write("\24\2\2\u01a8\u01a7\3\2\2\2\u01a8\u01a9\3\2\2\2\u01a9")
        buf.write("\u01ac\3\2\2\2\u01aa\u01ad\5z>\2\u01ab\u01ad\5R*\2\u01ac")
        buf.write("\u01aa\3\2\2\2\u01ac\u01ab\3\2\2\2\u01adY\3\2\2\2\u01ae")
        buf.write("\u01b4\7%\2\2\u01af\u01b0\7@\2\2\u01b0\u01b1\5\\/\2\u01b1")
        buf.write("\u01b2\7A\2\2\u01b2\u01b4\3\2\2\2\u01b3\u01ae\3\2\2\2")
        buf.write("\u01b3\u01af\3\2\2\2\u01b4[\3\2\2\2\u01b5\u01ba\5^\60")
        buf.write("\2\u01b6\u01b7\7\61\2\2\u01b7\u01b9\5^\60\2\u01b8\u01b6")
        buf.write("\3\2\2\2\u01b9\u01bc\3\2\2\2\u01ba\u01b8\3\2\2\2\u01ba")
        buf.write("\u01bb\3\2\2\2\u01bb]\3\2\2\2\u01bc\u01ba\3\2\2\2\u01bd")
        buf.write("\u01c3\5|?\2\u01be\u01c3\7%\2\2\u01bf\u01c0\7&\2\2\u01c0")
        buf.write("\u01c1\7.\2\2\u01c1\u01c3\7&\2\2\u01c2\u01bd\3\2\2\2\u01c2")
        buf.write("\u01be\3\2\2\2\u01c2\u01bf\3\2\2\2\u01c3_\3\2\2\2\u01c4")
        buf.write("\u01c5\5t;\2\u01c5\u01c6\7,\2\2\u01c6\u01c7\5l\67\2\u01c7")
        buf.write("a\3\2\2\2\u01c8\u01c9\7@\2\2\u01c9\u01cc\5t;\2\u01ca\u01cb")
        buf.write("\7*\2\2\u01cb\u01cd\5n8\2\u01cc\u01ca\3\2\2\2\u01cc\u01cd")
        buf.write("\3\2\2\2\u01cd\u01ce\3\2\2\2\u01ce\u01cf\7A\2\2\u01cf")
        buf.write("c\3\2\2\2\u01d0\u01d4\5f\64\2\u01d1\u01d4\5h\65\2\u01d2")
        buf.write("\u01d4\5j\66\2\u01d3\u01d0\3\2\2\2\u01d3\u01d1\3\2\2\2")
        buf.write("\u01d3\u01d2\3\2\2\2\u01d4e\3\2\2\2\u01d5\u01d6\7%\2\2")
        buf.write("\u01d6\u01d7\7D\2\2\u01d7\u01d8\5b\62\2\u01d8\u01d9\7")
        buf.write("E\2\2\u01d9g\3\2\2\2\u01da\u01db\7%\2\2\u01db\u01dc\7")
        buf.write("D\2\2\u01dc\u01dd\5|?\2\u01dd\u01de\7\61\2\2\u01de\u01df")
        buf.write("\5b\62\2\u01df\u01e0\7E\2\2\u01e0i\3\2\2\2\u01e1\u01e2")
        buf.write("\7%\2\2\u01e2\u01e3\7D\2\2\u01e3\u01e4\7$\2\2\u01e4\u01e5")
        buf.write("\7\61\2\2\u01e5\u01e6\5r:\2\u01e6\u01e7\7\61\2\2\u01e7")
        buf.write("\u01e8\5b\62\2\u01e8k\3\2\2\2\u01e9\u01ee\5n8\2\u01ea")
        buf.write("\u01eb\7\62\2\2\u01eb\u01ed\5n8\2\u01ec\u01ea\3\2\2\2")
        buf.write("\u01ed\u01f0\3\2\2\2\u01ee\u01ec\3\2\2\2\u01ee\u01ef\3")
        buf.write("\2\2\2\u01efm\3\2\2\2\u01f0\u01ee\3\2\2\2\u01f1\u01f6")
        buf.write("\5p9\2\u01f2\u01f3\7\61\2\2\u01f3\u01f5\5p9\2\u01f4\u01f2")
        buf.write("\3\2\2\2\u01f5\u01f8\3\2\2\2\u01f6\u01f4\3\2\2\2\u01f6")
        buf.write("\u01f7\3\2\2\2\u01f7o\3\2\2\2\u01f8\u01f6\3\2\2\2\u01f9")
        buf.write("\u01fb\7\16\2\2\u01fa\u01f9\3\2\2\2\u01fa\u01fb\3\2\2")
        buf.write("\2\u01fb\u01fc\3\2\2\2\u01fc\u0212\7%\2\2\u01fd\u01ff")
        buf.write("\7\16\2\2\u01fe\u01fd\3\2\2\2\u01fe\u01ff\3\2\2\2\u01ff")
        buf.write("\u0200\3\2\2\2\u0200\u0212\5r:\2\u0201\u0202\7\16\2\2")
        buf.write("\u0202\u0212\5b\62\2\u0203\u0204\5z>\2\u0204\u0205\t\5")
        buf.write("\2\2\u0205\u0206\5z>\2\u0206\u0212\3\2\2\2\u0207\u0208")
        buf.write("\5z>\2\u0208\u020b\7\r\2\2\u0209\u020c\5d\63\2\u020a\u020c")
        buf.write("\5r:\2\u020b\u0209\3\2\2\2\u020b\u020a\3\2\2\2\u020c\u0212")
        buf.write("\3\2\2\2\u020d\u020e\5v<\2\u020e\u020f\5\u0080A\2\u020f")
        buf.write("\u0210\5v<\2\u0210\u0212\3\2\2\2\u0211\u01fa\3\2\2\2\u0211")
        buf.write("\u01fe\3\2\2\2\u0211\u0201\3\2\2\2\u0211\u0203\3\2\2\2")
        buf.write("\u0211\u0207\3\2\2\2\u0211\u020d\3\2\2\2\u0212q\3\2\2")
        buf.write("\2\u0213\u0214\5z>\2\u0214\u0215\7D\2\2\u0215\u021a\5")
        buf.write("r:\2\u0216\u0217\7\61\2\2\u0217\u0219\5r:\2\u0218\u0216")
        buf.write("\3\2\2\2\u0219\u021c\3\2\2\2\u021a\u0218\3\2\2\2\u021a")
        buf.write("\u021b\3\2\2\2\u021b\u021d\3\2\2\2\u021c\u021a\3\2\2\2")
        buf.write("\u021d\u021e\7E\2\2\u021e\u0221\3\2\2\2\u021f\u0221\5")
        buf.write("x=\2\u0220\u0213\3\2\2\2\u0220\u021f\3\2\2\2\u0221s\3")
        buf.write("\2\2\2\u0222\u0227\5r:\2\u0223\u0224\7\61\2\2\u0224\u0226")
        buf.write("\5r:\2\u0225\u0223\3\2\2\2\u0226\u0229\3\2\2\2\u0227\u0225")
        buf.write("\3\2\2\2\u0227\u0228\3\2\2\2\u0228u\3\2\2\2\u0229\u0227")
        buf.write("\3\2\2\2\u022a\u022b\b<\1\2\u022b\u022c\7D\2\2\u022c\u022d")
        buf.write("\5v<\2\u022d\u022e\7E\2\2\u022e\u0234\3\2\2\2\u022f\u0232")
        buf.write("\5x=\2\u0230\u0232\5d\63\2\u0231\u022f\3\2\2\2\u0231\u0230")
        buf.write("\3\2\2\2\u0232\u0234\3\2\2\2\u0233\u022a\3\2\2\2\u0233")
        buf.write("\u0231\3\2\2\2\u0234\u0240\3\2\2\2\u0235\u0236\f\6\2\2")
        buf.write("\u0236\u0237\t\6\2\2\u0237\u023f\5v<\7\u0238\u0239\f\5")
        buf.write("\2\2\u0239\u023a\7=\2\2\u023a\u023f\5v<\6\u023b\u023c")
        buf.write("\f\4\2\2\u023c\u023d\t\7\2\2\u023d\u023f\5v<\5\u023e\u0235")
        buf.write("\3\2\2\2\u023e\u0238\3\2\2\2\u023e\u023b\3\2\2\2\u023f")
        buf.write("\u0242\3\2\2\2\u0240\u023e\3\2\2\2\u0240\u0241\3\2\2\2")
        buf.write("\u0241w\3\2\2\2\u0242\u0240\3\2\2\2\u0243\u0246\5z>\2")
        buf.write("\u0244\u0246\5|?\2\u0245\u0243\3\2\2\2\u0245\u0244\3\2")
        buf.write("\2\2\u0246y\3\2\2\2\u0247\u024c\7%\2\2\u0248\u0249\7/")
        buf.write("\2\2\u0249\u024b\7%\2\2\u024a\u0248\3\2\2\2\u024b\u024e")
        buf.write("\3\2\2\2\u024c\u024a\3\2\2\2\u024c\u024d\3\2\2\2\u024d")
        buf.write("{\3\2\2\2\u024e\u024c\3\2\2\2\u024f\u0250\t\b\2\2\u0250")
        buf.write("}\3\2\2\2\u0251\u0252\t\t\2\2\u0252\177\3\2\2\2\u0253")
        buf.write("\u0254\t\n\2\2\u0254\u0081\3\2\2\2\u0255\u0256\t\13\2")
        buf.write("\2\u0256\u0083\3\2\2\2?\u0091\u009a\u009d\u00a9\u00b0")
        buf.write("\u00b7\u00bd\u00c1\u00ca\u00d1\u00db\u00e4\u00eb\u00ee")
        buf.write("\u00fc\u0109\u010d\u0116\u011d\u0122\u0125\u0137\u013d")
        buf.write("\u0140\u014a\u0151\u0158\u015f\u0165\u016b\u0172\u0176")
        buf.write("\u017e\u0183\u018c\u0193\u019b\u01a0\u01a5\u01a8\u01ac")
        buf.write("\u01b3\u01ba\u01c2\u01cc\u01d3\u01ee\u01f6\u01fa\u01fe")
        buf.write("\u020b\u0211\u021a\u0220\u0227\u0231\u0233\u023e\u0240")
        buf.write("\u0245\u024c")
        return buf.getvalue()


class FormulaParser ( Parser ):

    grammarFileName = "Formula.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "'domain'", "'model'", "'transform'", 
                     "'system'", "'includes'", "'extends'", "'of'", "'returns'", 
                     "'at'", "'machine'", "'is'", "'no'", "'new'", "'fun'", 
                     "'inj'", "'bij'", "'sur'", "'any'", "'sub'", "'ensures'", 
                     "'requires'", "'conforms'", "'some'", "'atleast'", 
                     "'atmost'", "'partial'", "'initially'", "'next'", "'property'", 
                     "'boot'", "'import'", "'from'", "'as'", "<INVALID>", 
                     "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                     "<INVALID>", "'|'", "'::='", "':-'", "'::'", "'..'", 
                     "'.'", "':'", "','", "';'", "'='", "'!='", "'<='", 
                     "'>='", "'<'", "'>'", "'+'", "'-'", "'*'", "'/'", "'%'", 
                     "'=>'", "'->'", "'{'", "'}'", "'['", "']'", "'('", 
                     "')'" ]

    symbolicNames = [ "<INVALID>", "DOMAIN", "MODEL", "TRANSFORM", "SYSTEM", 
                      "INCLUDES", "EXTENDS", "OF", "RETURNS", "AT", "MACHINE", 
                      "IS", "NO", "NEW", "FUN", "INJ", "BIJ", "SUR", "ANY", 
                      "SUB", "ENSURES", "REQUIRES", "CONFORMS", "SOME", 
                      "ATLEAST", "ATMOST", "PARTIAL", "INITIALLY", "NEXT", 
                      "PROPERTY", "BOOT", "IMPORT", "FROM", "AS", "TID", 
                      "BId", "DECIMAL", "REAL", "FRAC", "STRING", "PIPE", 
                      "TYPEDEF", "RULE", "RENAMES", "RANGE", "DOT", "COLON", 
                      "COMMA", "SEMICOLON", "EQ", "NE", "LE", "GE", "LT", 
                      "GT", "PLUS", "MINUS", "MUL", "DIV", "MOD", "STRONGARROW", 
                      "WEAKARROW", "LBRACE", "RBRACE", "LBRACKET", "RBRACKET", 
                      "LPAREN", "RPAREN", "WS", "BLOCKCOMMENT", "LINECOMMENT" ]

    RULE_config = 0
    RULE_sentenceConfig = 1
    RULE_settingList = 2
    RULE_setting = 3
    RULE_program = 4
    RULE_importModule = 5
    RULE_moduleList = 6
    RULE_module = 7
    RULE_modRefs = 8
    RULE_modRef = 9
    RULE_transform = 10
    RULE_transformSigConfig = 11
    RULE_transformSig = 12
    RULE_transSigIn = 13
    RULE_voMParamList = 14
    RULE_valOrModelParam = 15
    RULE_transSentenceConfig = 16
    RULE_transSentence = 17
    RULE_tSystem = 18
    RULE_transSteps = 19
    RULE_transStepConfig = 20
    RULE_step = 21
    RULE_model = 22
    RULE_modelBody = 23
    RULE_modelSentence = 24
    RULE_modelContractConf = 25
    RULE_modelContract = 26
    RULE_cardSpec = 27
    RULE_modelSigConfig = 28
    RULE_modelIntro = 29
    RULE_modelSig = 30
    RULE_modelFactList = 31
    RULE_modelFact = 32
    RULE_domain = 33
    RULE_domainSigConfig = 34
    RULE_domainSig = 35
    RULE_domSentences = 36
    RULE_domSentenceConfig = 37
    RULE_domSentence = 38
    RULE_typeDecl = 39
    RULE_unnBody = 40
    RULE_funcDecl = 41
    RULE_fields = 42
    RULE_field = 43
    RULE_unnElem = 44
    RULE_enumList = 45
    RULE_enumCnst = 46
    RULE_formulaRule = 47
    RULE_setComprehension = 48
    RULE_aggregation = 49
    RULE_oneArgAggregation = 50
    RULE_twoArgAggregation = 51
    RULE_threeArgAggregation = 52
    RULE_disjunction = 53
    RULE_conjunction = 54
    RULE_constraint = 55
    RULE_funcTerm = 56
    RULE_funcTermList = 57
    RULE_arithmeticTerm = 58
    RULE_atom = 59
    RULE_qualId = 60
    RULE_constant = 61
    RULE_binOp = 62
    RULE_relOp = 63
    RULE_funModifier = 64

    ruleNames =  [ "config", "sentenceConfig", "settingList", "setting", 
                   "program", "importModule", "moduleList", "module", "modRefs", 
                   "modRef", "transform", "transformSigConfig", "transformSig", 
                   "transSigIn", "voMParamList", "valOrModelParam", "transSentenceConfig", 
                   "transSentence", "tSystem", "transSteps", "transStepConfig", 
                   "step", "model", "modelBody", "modelSentence", "modelContractConf", 
                   "modelContract", "cardSpec", "modelSigConfig", "modelIntro", 
                   "modelSig", "modelFactList", "modelFact", "domain", "domainSigConfig", 
                   "domainSig", "domSentences", "domSentenceConfig", "domSentence", 
                   "typeDecl", "unnBody", "funcDecl", "fields", "field", 
                   "unnElem", "enumList", "enumCnst", "formulaRule", "setComprehension", 
                   "aggregation", "oneArgAggregation", "twoArgAggregation", 
                   "threeArgAggregation", "disjunction", "conjunction", 
                   "constraint", "funcTerm", "funcTermList", "arithmeticTerm", 
                   "atom", "qualId", "constant", "binOp", "relOp", "funModifier" ]

    EOF = Token.EOF
    DOMAIN=1
    MODEL=2
    TRANSFORM=3
    SYSTEM=4
    INCLUDES=5
    EXTENDS=6
    OF=7
    RETURNS=8
    AT=9
    MACHINE=10
    IS=11
    NO=12
    NEW=13
    FUN=14
    INJ=15
    BIJ=16
    SUR=17
    ANY=18
    SUB=19
    ENSURES=20
    REQUIRES=21
    CONFORMS=22
    SOME=23
    ATLEAST=24
    ATMOST=25
    PARTIAL=26
    INITIALLY=27
    NEXT=28
    PROPERTY=29
    BOOT=30
    IMPORT=31
    FROM=32
    AS=33
    TID=34
    BId=35
    DECIMAL=36
    REAL=37
    FRAC=38
    STRING=39
    PIPE=40
    TYPEDEF=41
    RULE=42
    RENAMES=43
    RANGE=44
    DOT=45
    COLON=46
    COMMA=47
    SEMICOLON=48
    EQ=49
    NE=50
    LE=51
    GE=52
    LT=53
    GT=54
    PLUS=55
    MINUS=56
    MUL=57
    DIV=58
    MOD=59
    STRONGARROW=60
    WEAKARROW=61
    LBRACE=62
    RBRACE=63
    LBRACKET=64
    RBRACKET=65
    LPAREN=66
    RPAREN=67
    WS=68
    BLOCKCOMMENT=69
    LINECOMMENT=70

    def __init__(self, input:TokenStream, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.7.2")
        self._interp = ParserATNSimulator(self, self.atn, self.decisionsToDFA, self.sharedContextCache)
        self._predicates = None




    class ConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def LBRACKET(self):
            return self.getToken(FormulaParser.LBRACKET, 0)

        def settingList(self):
            return self.getTypedRuleContext(FormulaParser.SettingListContext,0)


        def RBRACKET(self):
            return self.getToken(FormulaParser.RBRACKET, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_config

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterConfig" ):
                listener.enterConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitConfig" ):
                listener.exitConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitConfig" ):
                return visitor.visitConfig(self)
            else:
                return visitor.visitChildren(self)




    def config(self):

        localctx = FormulaParser.ConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 0, self.RULE_config)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 130
            self.match(FormulaParser.LBRACKET)
            self.state = 131
            self.settingList()
            self.state = 132
            self.match(FormulaParser.RBRACKET)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class SentenceConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def LBRACKET(self):
            return self.getToken(FormulaParser.LBRACKET, 0)

        def settingList(self):
            return self.getTypedRuleContext(FormulaParser.SettingListContext,0)


        def RBRACKET(self):
            return self.getToken(FormulaParser.RBRACKET, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_sentenceConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSentenceConfig" ):
                listener.enterSentenceConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSentenceConfig" ):
                listener.exitSentenceConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSentenceConfig" ):
                return visitor.visitSentenceConfig(self)
            else:
                return visitor.visitChildren(self)




    def sentenceConfig(self):

        localctx = FormulaParser.SentenceConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 2, self.RULE_sentenceConfig)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 134
            self.match(FormulaParser.LBRACKET)
            self.state = 135
            self.settingList()
            self.state = 136
            self.match(FormulaParser.RBRACKET)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class SettingListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def setting(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.SettingContext)
            else:
                return self.getTypedRuleContext(FormulaParser.SettingContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_settingList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSettingList" ):
                listener.enterSettingList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSettingList" ):
                listener.exitSettingList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSettingList" ):
                return visitor.visitSettingList(self)
            else:
                return visitor.visitChildren(self)




    def settingList(self):

        localctx = FormulaParser.SettingListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 4, self.RULE_settingList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 138
            self.setting()
            self.state = 143
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 139
                self.match(FormulaParser.COMMA)
                self.state = 140
                self.setting()
                self.state = 145
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class SettingContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def qualId(self):
            return self.getTypedRuleContext(FormulaParser.QualIdContext,0)


        def EQ(self):
            return self.getToken(FormulaParser.EQ, 0)

        def constant(self):
            return self.getTypedRuleContext(FormulaParser.ConstantContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_setting

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSetting" ):
                listener.enterSetting(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSetting" ):
                listener.exitSetting(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSetting" ):
                return visitor.visitSetting(self)
            else:
                return visitor.visitChildren(self)




    def setting(self):

        localctx = FormulaParser.SettingContext(self, self._ctx, self.state)
        self.enterRule(localctx, 6, self.RULE_setting)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 146
            self.qualId()
            self.state = 147
            self.match(FormulaParser.EQ)
            self.state = 148
            self.constant()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ProgramContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def EOF(self):
            return self.getToken(FormulaParser.EOF, 0)

        def moduleList(self):
            return self.getTypedRuleContext(FormulaParser.ModuleListContext,0)


        def importModule(self):
            return self.getTypedRuleContext(FormulaParser.ImportModuleContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_program

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterProgram" ):
                listener.enterProgram(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitProgram" ):
                listener.exitProgram(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitProgram" ):
                return visitor.visitProgram(self)
            else:
                return visitor.visitChildren(self)




    def program(self):

        localctx = FormulaParser.ProgramContext(self, self._ctx, self.state)
        self.enterRule(localctx, 8, self.RULE_program)
        self._la = 0 # Token type
        try:
            self.state = 155
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,2,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 150
                self.match(FormulaParser.EOF)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 152
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==FormulaParser.IMPORT:
                    self.state = 151
                    self.importModule()


                self.state = 154
                self.moduleList()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ImportModuleContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def IMPORT(self):
            return self.getToken(FormulaParser.IMPORT, 0)

        def BId(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.BId)
            else:
                return self.getToken(FormulaParser.BId, i)

        def FROM(self):
            return self.getToken(FormulaParser.FROM, 0)

        def STRING(self):
            return self.getToken(FormulaParser.STRING, 0)

        def AS(self):
            return self.getToken(FormulaParser.AS, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_importModule

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterImportModule" ):
                listener.enterImportModule(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitImportModule" ):
                listener.exitImportModule(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitImportModule" ):
                return visitor.visitImportModule(self)
            else:
                return visitor.visitChildren(self)




    def importModule(self):

        localctx = FormulaParser.ImportModuleContext(self, self._ctx, self.state)
        self.enterRule(localctx, 10, self.RULE_importModule)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 157
            self.match(FormulaParser.IMPORT)
            self.state = 158
            self.match(FormulaParser.BId)
            self.state = 159
            self.match(FormulaParser.FROM)
            self.state = 160
            self.match(FormulaParser.STRING)
            self.state = 161
            self.match(FormulaParser.AS)
            self.state = 162
            self.match(FormulaParser.BId)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModuleListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def module(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ModuleContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ModuleContext,i)


        def getRuleIndex(self):
            return FormulaParser.RULE_moduleList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModuleList" ):
                listener.enterModuleList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModuleList" ):
                listener.exitModuleList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModuleList" ):
                return visitor.visitModuleList(self)
            else:
                return visitor.visitChildren(self)




    def moduleList(self):

        localctx = FormulaParser.ModuleListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 12, self.RULE_moduleList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 167
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while (((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.DOMAIN) | (1 << FormulaParser.MODEL) | (1 << FormulaParser.TRANSFORM) | (1 << FormulaParser.PARTIAL))) != 0):
                self.state = 164
                self.module()
                self.state = 169
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModuleContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def domain(self):
            return self.getTypedRuleContext(FormulaParser.DomainContext,0)


        def model(self):
            return self.getTypedRuleContext(FormulaParser.ModelContext,0)


        def transform(self):
            return self.getTypedRuleContext(FormulaParser.TransformContext,0)


        def tSystem(self):
            return self.getTypedRuleContext(FormulaParser.TSystemContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_module

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModule" ):
                listener.enterModule(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModule" ):
                listener.exitModule(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModule" ):
                return visitor.visitModule(self)
            else:
                return visitor.visitChildren(self)




    def module(self):

        localctx = FormulaParser.ModuleContext(self, self._ctx, self.state)
        self.enterRule(localctx, 14, self.RULE_module)
        try:
            self.state = 174
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,4,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 170
                self.domain()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 171
                self.model()
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 172
                self.transform()
                pass

            elif la_ == 4:
                self.enterOuterAlt(localctx, 4)
                self.state = 173
                self.tSystem()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModRefsContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modRef(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ModRefContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ModRefContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_modRefs

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModRefs" ):
                listener.enterModRefs(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModRefs" ):
                listener.exitModRefs(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModRefs" ):
                return visitor.visitModRefs(self)
            else:
                return visitor.visitChildren(self)




    def modRefs(self):

        localctx = FormulaParser.ModRefsContext(self, self._ctx, self.state)
        self.enterRule(localctx, 16, self.RULE_modRefs)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 176
            self.modRef()
            self.state = 181
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 177
                self.match(FormulaParser.COMMA)
                self.state = 178
                self.modRef()
                self.state = 183
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModRefContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.BId)
            else:
                return self.getToken(FormulaParser.BId, i)

        def RENAMES(self):
            return self.getToken(FormulaParser.RENAMES, 0)

        def AT(self):
            return self.getToken(FormulaParser.AT, 0)

        def STRING(self):
            return self.getToken(FormulaParser.STRING, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_modRef

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModRef" ):
                listener.enterModRef(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModRef" ):
                listener.exitModRef(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModRef" ):
                return visitor.visitModRef(self)
            else:
                return visitor.visitChildren(self)




    def modRef(self):

        localctx = FormulaParser.ModRefContext(self, self._ctx, self.state)
        self.enterRule(localctx, 18, self.RULE_modRef)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 184
            self.match(FormulaParser.BId)
            self.state = 187
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.RENAMES:
                self.state = 185
                self.match(FormulaParser.RENAMES)
                self.state = 186
                self.match(FormulaParser.BId)


            self.state = 191
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.AT:
                self.state = 189
                self.match(FormulaParser.AT)
                self.state = 190
                self.match(FormulaParser.STRING)


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransformContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def TRANSFORM(self):
            return self.getToken(FormulaParser.TRANSFORM, 0)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def transformSigConfig(self):
            return self.getTypedRuleContext(FormulaParser.TransformSigConfigContext,0)


        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def transSentenceConfig(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.TransSentenceConfigContext)
            else:
                return self.getTypedRuleContext(FormulaParser.TransSentenceConfigContext,i)


        def getRuleIndex(self):
            return FormulaParser.RULE_transform

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransform" ):
                listener.enterTransform(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransform" ):
                listener.exitTransform(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransform" ):
                return visitor.visitTransform(self)
            else:
                return visitor.visitChildren(self)




    def transform(self):

        localctx = FormulaParser.TransformContext(self, self._ctx, self.state)
        self.enterRule(localctx, 20, self.RULE_transform)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 193
            self.match(FormulaParser.TRANSFORM)
            self.state = 194
            self.match(FormulaParser.BId)
            self.state = 195
            self.transformSigConfig()
            self.state = 196
            self.match(FormulaParser.LBRACE)
            self.state = 200
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while ((((_la - 20)) & ~0x3f) == 0 and ((1 << (_la - 20)) & ((1 << (FormulaParser.ENSURES - 20)) | (1 << (FormulaParser.REQUIRES - 20)) | (1 << (FormulaParser.BId - 20)) | (1 << (FormulaParser.DECIMAL - 20)) | (1 << (FormulaParser.REAL - 20)) | (1 << (FormulaParser.FRAC - 20)) | (1 << (FormulaParser.STRING - 20)) | (1 << (FormulaParser.LBRACKET - 20)))) != 0):
                self.state = 197
                self.transSentenceConfig()
                self.state = 202
                self._errHandler.sync(self)
                _la = self._input.LA(1)

            self.state = 203
            self.match(FormulaParser.RBRACE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransformSigConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def transformSig(self):
            return self.getTypedRuleContext(FormulaParser.TransformSigContext,0)


        def config(self):
            return self.getTypedRuleContext(FormulaParser.ConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_transformSigConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransformSigConfig" ):
                listener.enterTransformSigConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransformSigConfig" ):
                listener.exitTransformSigConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransformSigConfig" ):
                return visitor.visitTransformSigConfig(self)
            else:
                return visitor.visitChildren(self)




    def transformSigConfig(self):

        localctx = FormulaParser.TransformSigConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 22, self.RULE_transformSigConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 205
            self.transformSig()
            self.state = 207
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 206
                self.config()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransformSigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def transSigIn(self):
            return self.getTypedRuleContext(FormulaParser.TransSigInContext,0)


        def RETURNS(self):
            return self.getToken(FormulaParser.RETURNS, 0)

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def modRefs(self):
            return self.getTypedRuleContext(FormulaParser.ModRefsContext,0)


        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_transformSig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransformSig" ):
                listener.enterTransformSig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransformSig" ):
                listener.exitTransformSig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransformSig" ):
                return visitor.visitTransformSig(self)
            else:
                return visitor.visitChildren(self)




    def transformSig(self):

        localctx = FormulaParser.TransformSigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 24, self.RULE_transformSig)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 209
            self.transSigIn()
            self.state = 210
            self.match(FormulaParser.RETURNS)
            self.state = 211
            self.match(FormulaParser.LPAREN)
            self.state = 212
            self.modRefs()
            self.state = 213
            self.match(FormulaParser.RPAREN)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransSigInContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def voMParamList(self):
            return self.getTypedRuleContext(FormulaParser.VoMParamListContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_transSigIn

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransSigIn" ):
                listener.enterTransSigIn(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransSigIn" ):
                listener.exitTransSigIn(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransSigIn" ):
                return visitor.visitTransSigIn(self)
            else:
                return visitor.visitChildren(self)




    def transSigIn(self):

        localctx = FormulaParser.TransSigInContext(self, self._ctx, self.state)
        self.enterRule(localctx, 26, self.RULE_transSigIn)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 215
            self.match(FormulaParser.LPAREN)
            self.state = 217
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.BId:
                self.state = 216
                self.voMParamList()


            self.state = 219
            self.match(FormulaParser.RPAREN)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class VoMParamListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def valOrModelParam(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ValOrModelParamContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ValOrModelParamContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_voMParamList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterVoMParamList" ):
                listener.enterVoMParamList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitVoMParamList" ):
                listener.exitVoMParamList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitVoMParamList" ):
                return visitor.visitVoMParamList(self)
            else:
                return visitor.visitChildren(self)




    def voMParamList(self):

        localctx = FormulaParser.VoMParamListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 28, self.RULE_voMParamList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 221
            self.valOrModelParam()
            self.state = 226
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 222
                self.match(FormulaParser.COMMA)
                self.state = 223
                self.valOrModelParam()
                self.state = 228
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ValOrModelParamContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def COLON(self):
            return self.getToken(FormulaParser.COLON, 0)

        def unnBody(self):
            return self.getTypedRuleContext(FormulaParser.UnnBodyContext,0)


        def modRef(self):
            return self.getTypedRuleContext(FormulaParser.ModRefContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_valOrModelParam

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterValOrModelParam" ):
                listener.enterValOrModelParam(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitValOrModelParam" ):
                listener.exitValOrModelParam(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitValOrModelParam" ):
                return visitor.visitValOrModelParam(self)
            else:
                return visitor.visitChildren(self)




    def valOrModelParam(self):

        localctx = FormulaParser.ValOrModelParamContext(self, self._ctx, self.state)
        self.enterRule(localctx, 30, self.RULE_valOrModelParam)
        try:
            self.state = 233
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,12,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 229
                self.match(FormulaParser.BId)
                self.state = 230
                self.match(FormulaParser.COLON)
                self.state = 231
                self.unnBody()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 232
                self.modRef()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransSentenceConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def transSentence(self):
            return self.getTypedRuleContext(FormulaParser.TransSentenceContext,0)


        def sentenceConfig(self):
            return self.getTypedRuleContext(FormulaParser.SentenceConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_transSentenceConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransSentenceConfig" ):
                listener.enterTransSentenceConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransSentenceConfig" ):
                listener.exitTransSentenceConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransSentenceConfig" ):
                return visitor.visitTransSentenceConfig(self)
            else:
                return visitor.visitChildren(self)




    def transSentenceConfig(self):

        localctx = FormulaParser.TransSentenceConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 32, self.RULE_transSentenceConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 236
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 235
                self.sentenceConfig()


            self.state = 238
            self.transSentence()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransSentenceContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def formulaRule(self):
            return self.getTypedRuleContext(FormulaParser.FormulaRuleContext,0)


        def typeDecl(self):
            return self.getTypedRuleContext(FormulaParser.TypeDeclContext,0)


        def ENSURES(self):
            return self.getToken(FormulaParser.ENSURES, 0)

        def disjunction(self):
            return self.getTypedRuleContext(FormulaParser.DisjunctionContext,0)


        def DOT(self):
            return self.getToken(FormulaParser.DOT, 0)

        def REQUIRES(self):
            return self.getToken(FormulaParser.REQUIRES, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_transSentence

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransSentence" ):
                listener.enterTransSentence(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransSentence" ):
                listener.exitTransSentence(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransSentence" ):
                return visitor.visitTransSentence(self)
            else:
                return visitor.visitChildren(self)




    def transSentence(self):

        localctx = FormulaParser.TransSentenceContext(self, self._ctx, self.state)
        self.enterRule(localctx, 34, self.RULE_transSentence)
        try:
            self.state = 250
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,14,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 240
                self.formulaRule()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 241
                self.typeDecl()
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 242
                self.match(FormulaParser.ENSURES)
                self.state = 243
                self.disjunction()
                self.state = 244
                self.match(FormulaParser.DOT)
                pass

            elif la_ == 4:
                self.enterOuterAlt(localctx, 4)
                self.state = 246
                self.match(FormulaParser.REQUIRES)
                self.state = 247
                self.disjunction()
                self.state = 248
                self.match(FormulaParser.DOT)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TSystemContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def TRANSFORM(self):
            return self.getToken(FormulaParser.TRANSFORM, 0)

        def SYSTEM(self):
            return self.getToken(FormulaParser.SYSTEM, 0)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def transformSigConfig(self):
            return self.getTypedRuleContext(FormulaParser.TransformSigConfigContext,0)


        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def transSteps(self):
            return self.getTypedRuleContext(FormulaParser.TransStepsContext,0)


        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_tSystem

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTSystem" ):
                listener.enterTSystem(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTSystem" ):
                listener.exitTSystem(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTSystem" ):
                return visitor.visitTSystem(self)
            else:
                return visitor.visitChildren(self)




    def tSystem(self):

        localctx = FormulaParser.TSystemContext(self, self._ctx, self.state)
        self.enterRule(localctx, 36, self.RULE_tSystem)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 252
            self.match(FormulaParser.TRANSFORM)
            self.state = 253
            self.match(FormulaParser.SYSTEM)
            self.state = 254
            self.match(FormulaParser.BId)
            self.state = 255
            self.transformSigConfig()
            self.state = 256
            self.match(FormulaParser.LBRACE)
            self.state = 257
            self.transSteps()
            self.state = 258
            self.match(FormulaParser.RBRACE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransStepsContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def transStepConfig(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.TransStepConfigContext)
            else:
                return self.getTypedRuleContext(FormulaParser.TransStepConfigContext,i)


        def getRuleIndex(self):
            return FormulaParser.RULE_transSteps

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransSteps" ):
                listener.enterTransSteps(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransSteps" ):
                listener.exitTransSteps(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransSteps" ):
                return visitor.visitTransSteps(self)
            else:
                return visitor.visitChildren(self)




    def transSteps(self):

        localctx = FormulaParser.TransStepsContext(self, self._ctx, self.state)
        self.enterRule(localctx, 38, self.RULE_transSteps)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 263
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.BId or _la==FormulaParser.LBRACKET:
                self.state = 260
                self.transStepConfig()
                self.state = 265
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TransStepConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def step(self):
            return self.getTypedRuleContext(FormulaParser.StepContext,0)


        def sentenceConfig(self):
            return self.getTypedRuleContext(FormulaParser.SentenceConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_transStepConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTransStepConfig" ):
                listener.enterTransStepConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTransStepConfig" ):
                listener.exitTransStepConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTransStepConfig" ):
                return visitor.visitTransStepConfig(self)
            else:
                return visitor.visitChildren(self)




    def transStepConfig(self):

        localctx = FormulaParser.TransStepConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 40, self.RULE_transStepConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 267
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 266
                self.sentenceConfig()


            self.state = 269
            self.step()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class StepContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_step

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterStep" ):
                listener.enterStep(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitStep" ):
                listener.exitStep(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitStep" ):
                return visitor.visitStep(self)
            else:
                return visitor.visitChildren(self)




    def step(self):

        localctx = FormulaParser.StepContext(self, self._ctx, self.state)
        self.enterRule(localctx, 42, self.RULE_step)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 271
            self.match(FormulaParser.BId)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelSigConfig(self):
            return self.getTypedRuleContext(FormulaParser.ModelSigConfigContext,0)


        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def modelBody(self):
            return self.getTypedRuleContext(FormulaParser.ModelBodyContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_model

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModel" ):
                listener.enterModel(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModel" ):
                listener.exitModel(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModel" ):
                return visitor.visitModel(self)
            else:
                return visitor.visitChildren(self)




    def model(self):

        localctx = FormulaParser.ModelContext(self, self._ctx, self.state)
        self.enterRule(localctx, 44, self.RULE_model)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 273
            self.modelSigConfig()
            self.state = 274
            self.match(FormulaParser.LBRACE)
            self.state = 276
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,17,self._ctx)
            if la_ == 1:
                self.state = 275
                self.modelBody()


            self.state = 278
            self.match(FormulaParser.RBRACE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelBodyContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelSentence(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ModelSentenceContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ModelSentenceContext,i)


        def getRuleIndex(self):
            return FormulaParser.RULE_modelBody

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelBody" ):
                listener.enterModelBody(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelBody" ):
                listener.exitModelBody(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelBody" ):
                return visitor.visitModelBody(self)
            else:
                return visitor.visitChildren(self)




    def modelBody(self):

        localctx = FormulaParser.ModelBodyContext(self, self._ctx, self.state)
        self.enterRule(localctx, 46, self.RULE_modelBody)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 283
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while ((((_la - 20)) & ~0x3f) == 0 and ((1 << (_la - 20)) & ((1 << (FormulaParser.ENSURES - 20)) | (1 << (FormulaParser.REQUIRES - 20)) | (1 << (FormulaParser.BId - 20)) | (1 << (FormulaParser.DECIMAL - 20)) | (1 << (FormulaParser.REAL - 20)) | (1 << (FormulaParser.FRAC - 20)) | (1 << (FormulaParser.STRING - 20)) | (1 << (FormulaParser.LBRACKET - 20)))) != 0):
                self.state = 280
                self.modelSentence()
                self.state = 285
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelSentenceContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelFactList(self):
            return self.getTypedRuleContext(FormulaParser.ModelFactListContext,0)


        def modelContractConf(self):
            return self.getTypedRuleContext(FormulaParser.ModelContractConfContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_modelSentence

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelSentence" ):
                listener.enterModelSentence(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelSentence" ):
                listener.exitModelSentence(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelSentence" ):
                return visitor.visitModelSentence(self)
            else:
                return visitor.visitChildren(self)




    def modelSentence(self):

        localctx = FormulaParser.ModelSentenceContext(self, self._ctx, self.state)
        self.enterRule(localctx, 48, self.RULE_modelSentence)
        try:
            self.state = 288
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [FormulaParser.BId, FormulaParser.DECIMAL, FormulaParser.REAL, FormulaParser.FRAC, FormulaParser.STRING]:
                self.enterOuterAlt(localctx, 1)
                self.state = 286
                self.modelFactList()
                pass
            elif token in [FormulaParser.ENSURES, FormulaParser.REQUIRES, FormulaParser.LBRACKET]:
                self.enterOuterAlt(localctx, 2)
                self.state = 287
                self.modelContractConf()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelContractConfContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelContract(self):
            return self.getTypedRuleContext(FormulaParser.ModelContractContext,0)


        def sentenceConfig(self):
            return self.getTypedRuleContext(FormulaParser.SentenceConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_modelContractConf

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelContractConf" ):
                listener.enterModelContractConf(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelContractConf" ):
                listener.exitModelContractConf(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelContractConf" ):
                return visitor.visitModelContractConf(self)
            else:
                return visitor.visitChildren(self)




    def modelContractConf(self):

        localctx = FormulaParser.ModelContractConfContext(self, self._ctx, self.state)
        self.enterRule(localctx, 50, self.RULE_modelContractConf)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 291
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 290
                self.sentenceConfig()


            self.state = 293
            self.modelContract()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelContractContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def ENSURES(self):
            return self.getToken(FormulaParser.ENSURES, 0)

        def disjunction(self):
            return self.getTypedRuleContext(FormulaParser.DisjunctionContext,0)


        def DOT(self):
            return self.getToken(FormulaParser.DOT, 0)

        def REQUIRES(self):
            return self.getToken(FormulaParser.REQUIRES, 0)

        def cardSpec(self):
            return self.getTypedRuleContext(FormulaParser.CardSpecContext,0)


        def DECIMAL(self):
            return self.getToken(FormulaParser.DECIMAL, 0)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_modelContract

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelContract" ):
                listener.enterModelContract(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelContract" ):
                listener.exitModelContract(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelContract" ):
                return visitor.visitModelContract(self)
            else:
                return visitor.visitChildren(self)




    def modelContract(self):

        localctx = FormulaParser.ModelContractContext(self, self._ctx, self.state)
        self.enterRule(localctx, 52, self.RULE_modelContract)
        try:
            self.state = 309
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,21,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 295
                self.match(FormulaParser.ENSURES)
                self.state = 296
                self.disjunction()
                self.state = 297
                self.match(FormulaParser.DOT)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 299
                self.match(FormulaParser.REQUIRES)
                self.state = 300
                self.disjunction()
                self.state = 301
                self.match(FormulaParser.DOT)
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 303
                self.match(FormulaParser.REQUIRES)
                self.state = 304
                self.cardSpec()
                self.state = 305
                self.match(FormulaParser.DECIMAL)
                self.state = 306
                self.match(FormulaParser.BId)
                self.state = 307
                self.match(FormulaParser.DOT)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class CardSpecContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def SOME(self):
            return self.getToken(FormulaParser.SOME, 0)

        def ATMOST(self):
            return self.getToken(FormulaParser.ATMOST, 0)

        def ATLEAST(self):
            return self.getToken(FormulaParser.ATLEAST, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_cardSpec

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterCardSpec" ):
                listener.enterCardSpec(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitCardSpec" ):
                listener.exitCardSpec(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitCardSpec" ):
                return visitor.visitCardSpec(self)
            else:
                return visitor.visitChildren(self)




    def cardSpec(self):

        localctx = FormulaParser.CardSpecContext(self, self._ctx, self.state)
        self.enterRule(localctx, 54, self.RULE_cardSpec)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 311
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.SOME) | (1 << FormulaParser.ATLEAST) | (1 << FormulaParser.ATMOST))) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelSigConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelSig(self):
            return self.getTypedRuleContext(FormulaParser.ModelSigContext,0)


        def config(self):
            return self.getTypedRuleContext(FormulaParser.ConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_modelSigConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelSigConfig" ):
                listener.enterModelSigConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelSigConfig" ):
                listener.exitModelSigConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelSigConfig" ):
                return visitor.visitModelSigConfig(self)
            else:
                return visitor.visitChildren(self)




    def modelSigConfig(self):

        localctx = FormulaParser.ModelSigConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 56, self.RULE_modelSigConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 313
            self.modelSig()
            self.state = 315
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 314
                self.config()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelIntroContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def MODEL(self):
            return self.getToken(FormulaParser.MODEL, 0)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def OF(self):
            return self.getToken(FormulaParser.OF, 0)

        def modRef(self):
            return self.getTypedRuleContext(FormulaParser.ModRefContext,0)


        def PARTIAL(self):
            return self.getToken(FormulaParser.PARTIAL, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_modelIntro

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelIntro" ):
                listener.enterModelIntro(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelIntro" ):
                listener.exitModelIntro(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelIntro" ):
                return visitor.visitModelIntro(self)
            else:
                return visitor.visitChildren(self)




    def modelIntro(self):

        localctx = FormulaParser.ModelIntroContext(self, self._ctx, self.state)
        self.enterRule(localctx, 58, self.RULE_modelIntro)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 318
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.PARTIAL:
                self.state = 317
                self.match(FormulaParser.PARTIAL)


            self.state = 320
            self.match(FormulaParser.MODEL)
            self.state = 321
            self.match(FormulaParser.BId)
            self.state = 322
            self.match(FormulaParser.OF)
            self.state = 323
            self.modRef()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelSigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelIntro(self):
            return self.getTypedRuleContext(FormulaParser.ModelIntroContext,0)


        def modRefs(self):
            return self.getTypedRuleContext(FormulaParser.ModRefsContext,0)


        def INCLUDES(self):
            return self.getToken(FormulaParser.INCLUDES, 0)

        def EXTENDS(self):
            return self.getToken(FormulaParser.EXTENDS, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_modelSig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelSig" ):
                listener.enterModelSig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelSig" ):
                listener.exitModelSig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelSig" ):
                return visitor.visitModelSig(self)
            else:
                return visitor.visitChildren(self)




    def modelSig(self):

        localctx = FormulaParser.ModelSigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 60, self.RULE_modelSig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 325
            self.modelIntro()
            self.state = 328
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.INCLUDES or _la==FormulaParser.EXTENDS:
                self.state = 326
                _la = self._input.LA(1)
                if not(_la==FormulaParser.INCLUDES or _la==FormulaParser.EXTENDS):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 327
                self.modRefs()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelFactListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def modelFact(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ModelFactContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ModelFactContext,i)


        def DOT(self):
            return self.getToken(FormulaParser.DOT, 0)

        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_modelFactList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelFactList" ):
                listener.enterModelFactList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelFactList" ):
                listener.exitModelFactList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelFactList" ):
                return visitor.visitModelFactList(self)
            else:
                return visitor.visitChildren(self)




    def modelFactList(self):

        localctx = FormulaParser.ModelFactListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 62, self.RULE_modelFactList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 330
            self.modelFact()
            self.state = 335
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 331
                self.match(FormulaParser.COMMA)
                self.state = 332
                self.modelFact()
                self.state = 337
                self._errHandler.sync(self)
                _la = self._input.LA(1)

            self.state = 338
            self.match(FormulaParser.DOT)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModelFactContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def funcTerm(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermContext,0)


        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def IS(self):
            return self.getToken(FormulaParser.IS, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_modelFact

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModelFact" ):
                listener.enterModelFact(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModelFact" ):
                listener.exitModelFact(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModelFact" ):
                return visitor.visitModelFact(self)
            else:
                return visitor.visitChildren(self)




    def modelFact(self):

        localctx = FormulaParser.ModelFactContext(self, self._ctx, self.state)
        self.enterRule(localctx, 64, self.RULE_modelFact)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 342
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,26,self._ctx)
            if la_ == 1:
                self.state = 340
                self.match(FormulaParser.BId)
                self.state = 341
                self.match(FormulaParser.IS)


            self.state = 344
            self.funcTerm()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomainContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def domainSigConfig(self):
            return self.getTypedRuleContext(FormulaParser.DomainSigConfigContext,0)


        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def domSentences(self):
            return self.getTypedRuleContext(FormulaParser.DomSentencesContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_domain

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomain" ):
                listener.enterDomain(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomain" ):
                listener.exitDomain(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomain" ):
                return visitor.visitDomain(self)
            else:
                return visitor.visitChildren(self)




    def domain(self):

        localctx = FormulaParser.DomainContext(self, self._ctx, self.state)
        self.enterRule(localctx, 66, self.RULE_domain)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 346
            self.domainSigConfig()
            self.state = 347
            self.match(FormulaParser.LBRACE)
            self.state = 349
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,27,self._ctx)
            if la_ == 1:
                self.state = 348
                self.domSentences()


            self.state = 351
            self.match(FormulaParser.RBRACE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomainSigConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def domainSig(self):
            return self.getTypedRuleContext(FormulaParser.DomainSigContext,0)


        def config(self):
            return self.getTypedRuleContext(FormulaParser.ConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_domainSigConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomainSigConfig" ):
                listener.enterDomainSigConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomainSigConfig" ):
                listener.exitDomainSigConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomainSigConfig" ):
                return visitor.visitDomainSigConfig(self)
            else:
                return visitor.visitChildren(self)




    def domainSigConfig(self):

        localctx = FormulaParser.DomainSigConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 68, self.RULE_domainSigConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 353
            self.domainSig()
            self.state = 355
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 354
                self.config()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomainSigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def DOMAIN(self):
            return self.getToken(FormulaParser.DOMAIN, 0)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def modRefs(self):
            return self.getTypedRuleContext(FormulaParser.ModRefsContext,0)


        def EXTENDS(self):
            return self.getToken(FormulaParser.EXTENDS, 0)

        def INCLUDES(self):
            return self.getToken(FormulaParser.INCLUDES, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_domainSig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomainSig" ):
                listener.enterDomainSig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomainSig" ):
                listener.exitDomainSig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomainSig" ):
                return visitor.visitDomainSig(self)
            else:
                return visitor.visitChildren(self)




    def domainSig(self):

        localctx = FormulaParser.DomainSigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 70, self.RULE_domainSig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 357
            self.match(FormulaParser.DOMAIN)
            self.state = 358
            self.match(FormulaParser.BId)
            self.state = 361
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.INCLUDES or _la==FormulaParser.EXTENDS:
                self.state = 359
                _la = self._input.LA(1)
                if not(_la==FormulaParser.INCLUDES or _la==FormulaParser.EXTENDS):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 360
                self.modRefs()


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomSentencesContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def domSentenceConfig(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.DomSentenceConfigContext)
            else:
                return self.getTypedRuleContext(FormulaParser.DomSentenceConfigContext,i)


        def DOT(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.DOT)
            else:
                return self.getToken(FormulaParser.DOT, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_domSentences

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomSentences" ):
                listener.enterDomSentences(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomSentences" ):
                listener.exitDomSentences(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomSentences" ):
                return visitor.visitDomSentences(self)
            else:
                return visitor.visitChildren(self)




    def domSentences(self):

        localctx = FormulaParser.DomSentencesContext(self, self._ctx, self.state)
        self.enterRule(localctx, 72, self.RULE_domSentences)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 368
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while ((((_la - 22)) & ~0x3f) == 0 and ((1 << (_la - 22)) & ((1 << (FormulaParser.CONFORMS - 22)) | (1 << (FormulaParser.BId - 22)) | (1 << (FormulaParser.DECIMAL - 22)) | (1 << (FormulaParser.REAL - 22)) | (1 << (FormulaParser.FRAC - 22)) | (1 << (FormulaParser.STRING - 22)) | (1 << (FormulaParser.LBRACKET - 22)))) != 0):
                self.state = 363
                self.domSentenceConfig()
                self.state = 364
                self.match(FormulaParser.DOT)
                self.state = 370
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomSentenceConfigContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def domSentence(self):
            return self.getTypedRuleContext(FormulaParser.DomSentenceContext,0)


        def sentenceConfig(self):
            return self.getTypedRuleContext(FormulaParser.SentenceConfigContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_domSentenceConfig

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomSentenceConfig" ):
                listener.enterDomSentenceConfig(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomSentenceConfig" ):
                listener.exitDomSentenceConfig(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomSentenceConfig" ):
                return visitor.visitDomSentenceConfig(self)
            else:
                return visitor.visitChildren(self)




    def domSentenceConfig(self):

        localctx = FormulaParser.DomSentenceConfigContext(self, self._ctx, self.state)
        self.enterRule(localctx, 74, self.RULE_domSentenceConfig)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 372
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.LBRACKET:
                self.state = 371
                self.sentenceConfig()


            self.state = 374
            self.domSentence()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DomSentenceContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def typeDecl(self):
            return self.getTypedRuleContext(FormulaParser.TypeDeclContext,0)


        def formulaRule(self):
            return self.getTypedRuleContext(FormulaParser.FormulaRuleContext,0)


        def CONFORMS(self):
            return self.getToken(FormulaParser.CONFORMS, 0)

        def disjunction(self):
            return self.getTypedRuleContext(FormulaParser.DisjunctionContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_domSentence

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDomSentence" ):
                listener.enterDomSentence(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDomSentence" ):
                listener.exitDomSentence(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDomSentence" ):
                return visitor.visitDomSentence(self)
            else:
                return visitor.visitChildren(self)




    def domSentence(self):

        localctx = FormulaParser.DomSentenceContext(self, self._ctx, self.state)
        self.enterRule(localctx, 76, self.RULE_domSentence)
        try:
            self.state = 380
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,32,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 376
                self.typeDecl()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 377
                self.formulaRule()
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 378
                self.match(FormulaParser.CONFORMS)
                self.state = 379
                self.disjunction()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TypeDeclContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return FormulaParser.RULE_typeDecl

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class UnionTypeDeclContext(TypeDeclContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.TypeDeclContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)
        def TYPEDEF(self):
            return self.getToken(FormulaParser.TYPEDEF, 0)
        def unnBody(self):
            return self.getTypedRuleContext(FormulaParser.UnnBodyContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnionTypeDecl" ):
                listener.enterUnionTypeDecl(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnionTypeDecl" ):
                listener.exitUnionTypeDecl(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnionTypeDecl" ):
                return visitor.visitUnionTypeDecl(self)
            else:
                return visitor.visitChildren(self)


    class RegularTypeDeclContext(TypeDeclContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.TypeDeclContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)
        def TYPEDEF(self):
            return self.getToken(FormulaParser.TYPEDEF, 0)
        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)
        def fields(self):
            return self.getTypedRuleContext(FormulaParser.FieldsContext,0)

        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)
        def funcDecl(self):
            return self.getTypedRuleContext(FormulaParser.FuncDeclContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterRegularTypeDecl" ):
                listener.enterRegularTypeDecl(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitRegularTypeDecl" ):
                listener.exitRegularTypeDecl(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitRegularTypeDecl" ):
                return visitor.visitRegularTypeDecl(self)
            else:
                return visitor.visitChildren(self)



    def typeDecl(self):

        localctx = FormulaParser.TypeDeclContext(self, self._ctx, self.state)
        self.enterRule(localctx, 78, self.RULE_typeDecl)
        self._la = 0 # Token type
        try:
            self.state = 394
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,34,self._ctx)
            if la_ == 1:
                localctx = FormulaParser.RegularTypeDeclContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 382
                self.match(FormulaParser.BId)
                self.state = 383
                self.match(FormulaParser.TYPEDEF)
                self.state = 385
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if (((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.NEW) | (1 << FormulaParser.FUN) | (1 << FormulaParser.INJ) | (1 << FormulaParser.BIJ) | (1 << FormulaParser.SUR) | (1 << FormulaParser.SUB))) != 0):
                    self.state = 384
                    self.funcDecl()


                self.state = 387
                self.match(FormulaParser.LPAREN)
                self.state = 388
                self.fields()
                self.state = 389
                self.match(FormulaParser.RPAREN)
                pass

            elif la_ == 2:
                localctx = FormulaParser.UnionTypeDeclContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 391
                self.match(FormulaParser.BId)
                self.state = 392
                self.match(FormulaParser.TYPEDEF)
                self.state = 393
                self.unnBody()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class UnnBodyContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def unnElem(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.UnnElemContext)
            else:
                return self.getTypedRuleContext(FormulaParser.UnnElemContext,i)


        def PLUS(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.PLUS)
            else:
                return self.getToken(FormulaParser.PLUS, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_unnBody

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnnBody" ):
                listener.enterUnnBody(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnnBody" ):
                listener.exitUnnBody(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnnBody" ):
                return visitor.visitUnnBody(self)
            else:
                return visitor.visitChildren(self)




    def unnBody(self):

        localctx = FormulaParser.UnnBodyContext(self, self._ctx, self.state)
        self.enterRule(localctx, 80, self.RULE_unnBody)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 396
            self.unnElem()
            self.state = 401
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.PLUS:
                self.state = 397
                self.match(FormulaParser.PLUS)
                self.state = 398
                self.unnElem()
                self.state = 403
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FuncDeclContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def INJ(self):
            return self.getToken(FormulaParser.INJ, 0)

        def BIJ(self):
            return self.getToken(FormulaParser.BIJ, 0)

        def SUR(self):
            return self.getToken(FormulaParser.SUR, 0)

        def FUN(self):
            return self.getToken(FormulaParser.FUN, 0)

        def SUB(self):
            return self.getToken(FormulaParser.SUB, 0)

        def NEW(self):
            return self.getToken(FormulaParser.NEW, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_funcDecl

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFuncDecl" ):
                listener.enterFuncDecl(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFuncDecl" ):
                listener.exitFuncDecl(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFuncDecl" ):
                return visitor.visitFuncDecl(self)
            else:
                return visitor.visitChildren(self)




    def funcDecl(self):

        localctx = FormulaParser.FuncDeclContext(self, self._ctx, self.state)
        self.enterRule(localctx, 82, self.RULE_funcDecl)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 404
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.NEW) | (1 << FormulaParser.FUN) | (1 << FormulaParser.INJ) | (1 << FormulaParser.BIJ) | (1 << FormulaParser.SUR) | (1 << FormulaParser.SUB))) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FieldsContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def field(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.FieldContext)
            else:
                return self.getTypedRuleContext(FormulaParser.FieldContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def funModifier(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.FunModifierContext)
            else:
                return self.getTypedRuleContext(FormulaParser.FunModifierContext,i)


        def getRuleIndex(self):
            return FormulaParser.RULE_fields

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFields" ):
                listener.enterFields(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFields" ):
                listener.exitFields(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFields" ):
                return visitor.visitFields(self)
            else:
                return visitor.visitChildren(self)




    def fields(self):

        localctx = FormulaParser.FieldsContext(self, self._ctx, self.state)
        self.enterRule(localctx, 84, self.RULE_fields)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 406
            self.field()
            self.state = 414
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while (((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.COMMA) | (1 << FormulaParser.STRONGARROW) | (1 << FormulaParser.WEAKARROW))) != 0):
                self.state = 409
                self._errHandler.sync(self)
                token = self._input.LA(1)
                if token in [FormulaParser.COMMA]:
                    self.state = 407
                    self.match(FormulaParser.COMMA)
                    pass
                elif token in [FormulaParser.STRONGARROW, FormulaParser.WEAKARROW]:
                    self.state = 408
                    self.funModifier()
                    pass
                else:
                    raise NoViableAltException(self)

                self.state = 411
                self.field()
                self.state = 416
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FieldContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def qualId(self):
            return self.getTypedRuleContext(FormulaParser.QualIdContext,0)


        def unnBody(self):
            return self.getTypedRuleContext(FormulaParser.UnnBodyContext,0)


        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def COLON(self):
            return self.getToken(FormulaParser.COLON, 0)

        def ANY(self):
            return self.getToken(FormulaParser.ANY, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_field

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterField" ):
                listener.enterField(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitField" ):
                listener.exitField(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitField" ):
                return visitor.visitField(self)
            else:
                return visitor.visitChildren(self)




    def field(self):

        localctx = FormulaParser.FieldContext(self, self._ctx, self.state)
        self.enterRule(localctx, 86, self.RULE_field)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 419
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,38,self._ctx)
            if la_ == 1:
                self.state = 417
                self.match(FormulaParser.BId)
                self.state = 418
                self.match(FormulaParser.COLON)


            self.state = 422
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.ANY:
                self.state = 421
                self.match(FormulaParser.ANY)


            self.state = 426
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,40,self._ctx)
            if la_ == 1:
                self.state = 424
                self.qualId()
                pass

            elif la_ == 2:
                self.state = 425
                self.unnBody()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class UnnElemContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def enumList(self):
            return self.getTypedRuleContext(FormulaParser.EnumListContext,0)


        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_unnElem

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnnElem" ):
                listener.enterUnnElem(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnnElem" ):
                listener.exitUnnElem(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnnElem" ):
                return visitor.visitUnnElem(self)
            else:
                return visitor.visitChildren(self)




    def unnElem(self):

        localctx = FormulaParser.UnnElemContext(self, self._ctx, self.state)
        self.enterRule(localctx, 88, self.RULE_unnElem)
        try:
            self.state = 433
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [FormulaParser.BId]:
                self.enterOuterAlt(localctx, 1)
                self.state = 428
                self.match(FormulaParser.BId)
                pass
            elif token in [FormulaParser.LBRACE]:
                self.enterOuterAlt(localctx, 2)
                self.state = 429
                self.match(FormulaParser.LBRACE)
                self.state = 430
                self.enumList()
                self.state = 431
                self.match(FormulaParser.RBRACE)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class EnumListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def enumCnst(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.EnumCnstContext)
            else:
                return self.getTypedRuleContext(FormulaParser.EnumCnstContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_enumList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEnumList" ):
                listener.enterEnumList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEnumList" ):
                listener.exitEnumList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEnumList" ):
                return visitor.visitEnumList(self)
            else:
                return visitor.visitChildren(self)




    def enumList(self):

        localctx = FormulaParser.EnumListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 90, self.RULE_enumList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 435
            self.enumCnst()
            self.state = 440
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 436
                self.match(FormulaParser.COMMA)
                self.state = 437
                self.enumCnst()
                self.state = 442
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class EnumCnstContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def constant(self):
            return self.getTypedRuleContext(FormulaParser.ConstantContext,0)


        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def DECIMAL(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.DECIMAL)
            else:
                return self.getToken(FormulaParser.DECIMAL, i)

        def RANGE(self):
            return self.getToken(FormulaParser.RANGE, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_enumCnst

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEnumCnst" ):
                listener.enterEnumCnst(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEnumCnst" ):
                listener.exitEnumCnst(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEnumCnst" ):
                return visitor.visitEnumCnst(self)
            else:
                return visitor.visitChildren(self)




    def enumCnst(self):

        localctx = FormulaParser.EnumCnstContext(self, self._ctx, self.state)
        self.enterRule(localctx, 92, self.RULE_enumCnst)
        try:
            self.state = 448
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,43,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 443
                self.constant()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 444
                self.match(FormulaParser.BId)
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 445
                self.match(FormulaParser.DECIMAL)
                self.state = 446
                self.match(FormulaParser.RANGE)
                self.state = 447
                self.match(FormulaParser.DECIMAL)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FormulaRuleContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def funcTermList(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermListContext,0)


        def RULE(self):
            return self.getToken(FormulaParser.RULE, 0)

        def disjunction(self):
            return self.getTypedRuleContext(FormulaParser.DisjunctionContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_formulaRule

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFormulaRule" ):
                listener.enterFormulaRule(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFormulaRule" ):
                listener.exitFormulaRule(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFormulaRule" ):
                return visitor.visitFormulaRule(self)
            else:
                return visitor.visitChildren(self)




    def formulaRule(self):

        localctx = FormulaParser.FormulaRuleContext(self, self._ctx, self.state)
        self.enterRule(localctx, 94, self.RULE_formulaRule)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 450
            self.funcTermList()
            self.state = 451
            self.match(FormulaParser.RULE)
            self.state = 452
            self.disjunction()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class SetComprehensionContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def LBRACE(self):
            return self.getToken(FormulaParser.LBRACE, 0)

        def funcTermList(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermListContext,0)


        def RBRACE(self):
            return self.getToken(FormulaParser.RBRACE, 0)

        def PIPE(self):
            return self.getToken(FormulaParser.PIPE, 0)

        def conjunction(self):
            return self.getTypedRuleContext(FormulaParser.ConjunctionContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_setComprehension

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSetComprehension" ):
                listener.enterSetComprehension(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSetComprehension" ):
                listener.exitSetComprehension(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSetComprehension" ):
                return visitor.visitSetComprehension(self)
            else:
                return visitor.visitChildren(self)




    def setComprehension(self):

        localctx = FormulaParser.SetComprehensionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 96, self.RULE_setComprehension)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 454
            self.match(FormulaParser.LBRACE)
            self.state = 455
            self.funcTermList()
            self.state = 458
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==FormulaParser.PIPE:
                self.state = 456
                self.match(FormulaParser.PIPE)
                self.state = 457
                self.conjunction()


            self.state = 460
            self.match(FormulaParser.RBRACE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class AggregationContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def oneArgAggregation(self):
            return self.getTypedRuleContext(FormulaParser.OneArgAggregationContext,0)


        def twoArgAggregation(self):
            return self.getTypedRuleContext(FormulaParser.TwoArgAggregationContext,0)


        def threeArgAggregation(self):
            return self.getTypedRuleContext(FormulaParser.ThreeArgAggregationContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_aggregation

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAggregation" ):
                listener.enterAggregation(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAggregation" ):
                listener.exitAggregation(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAggregation" ):
                return visitor.visitAggregation(self)
            else:
                return visitor.visitChildren(self)




    def aggregation(self):

        localctx = FormulaParser.AggregationContext(self, self._ctx, self.state)
        self.enterRule(localctx, 98, self.RULE_aggregation)
        try:
            self.state = 465
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,45,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 462
                self.oneArgAggregation()
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 463
                self.twoArgAggregation()
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 464
                self.threeArgAggregation()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class OneArgAggregationContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def setComprehension(self):
            return self.getTypedRuleContext(FormulaParser.SetComprehensionContext,0)


        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_oneArgAggregation

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterOneArgAggregation" ):
                listener.enterOneArgAggregation(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitOneArgAggregation" ):
                listener.exitOneArgAggregation(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitOneArgAggregation" ):
                return visitor.visitOneArgAggregation(self)
            else:
                return visitor.visitChildren(self)




    def oneArgAggregation(self):

        localctx = FormulaParser.OneArgAggregationContext(self, self._ctx, self.state)
        self.enterRule(localctx, 100, self.RULE_oneArgAggregation)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 467
            self.match(FormulaParser.BId)
            self.state = 468
            self.match(FormulaParser.LPAREN)
            self.state = 469
            self.setComprehension()
            self.state = 470
            self.match(FormulaParser.RPAREN)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TwoArgAggregationContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def constant(self):
            return self.getTypedRuleContext(FormulaParser.ConstantContext,0)


        def COMMA(self):
            return self.getToken(FormulaParser.COMMA, 0)

        def setComprehension(self):
            return self.getTypedRuleContext(FormulaParser.SetComprehensionContext,0)


        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_twoArgAggregation

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTwoArgAggregation" ):
                listener.enterTwoArgAggregation(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTwoArgAggregation" ):
                listener.exitTwoArgAggregation(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTwoArgAggregation" ):
                return visitor.visitTwoArgAggregation(self)
            else:
                return visitor.visitChildren(self)




    def twoArgAggregation(self):

        localctx = FormulaParser.TwoArgAggregationContext(self, self._ctx, self.state)
        self.enterRule(localctx, 102, self.RULE_twoArgAggregation)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 472
            self.match(FormulaParser.BId)
            self.state = 473
            self.match(FormulaParser.LPAREN)
            self.state = 474
            self.constant()
            self.state = 475
            self.match(FormulaParser.COMMA)
            self.state = 476
            self.setComprehension()
            self.state = 477
            self.match(FormulaParser.RPAREN)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ThreeArgAggregationContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def TID(self):
            return self.getToken(FormulaParser.TID, 0)

        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def funcTerm(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermContext,0)


        def setComprehension(self):
            return self.getTypedRuleContext(FormulaParser.SetComprehensionContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_threeArgAggregation

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterThreeArgAggregation" ):
                listener.enterThreeArgAggregation(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitThreeArgAggregation" ):
                listener.exitThreeArgAggregation(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitThreeArgAggregation" ):
                return visitor.visitThreeArgAggregation(self)
            else:
                return visitor.visitChildren(self)




    def threeArgAggregation(self):

        localctx = FormulaParser.ThreeArgAggregationContext(self, self._ctx, self.state)
        self.enterRule(localctx, 104, self.RULE_threeArgAggregation)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 479
            self.match(FormulaParser.BId)
            self.state = 480
            self.match(FormulaParser.LPAREN)
            self.state = 481
            self.match(FormulaParser.TID)
            self.state = 482
            self.match(FormulaParser.COMMA)
            self.state = 483
            self.funcTerm()
            self.state = 484
            self.match(FormulaParser.COMMA)
            self.state = 485
            self.setComprehension()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class DisjunctionContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def conjunction(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ConjunctionContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ConjunctionContext,i)


        def SEMICOLON(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.SEMICOLON)
            else:
                return self.getToken(FormulaParser.SEMICOLON, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_disjunction

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDisjunction" ):
                listener.enterDisjunction(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDisjunction" ):
                listener.exitDisjunction(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDisjunction" ):
                return visitor.visitDisjunction(self)
            else:
                return visitor.visitChildren(self)




    def disjunction(self):

        localctx = FormulaParser.DisjunctionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 106, self.RULE_disjunction)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 487
            self.conjunction()
            self.state = 492
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.SEMICOLON:
                self.state = 488
                self.match(FormulaParser.SEMICOLON)
                self.state = 489
                self.conjunction()
                self.state = 494
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ConjunctionContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def constraint(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ConstraintContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ConstraintContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_conjunction

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterConjunction" ):
                listener.enterConjunction(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitConjunction" ):
                listener.exitConjunction(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitConjunction" ):
                return visitor.visitConjunction(self)
            else:
                return visitor.visitChildren(self)




    def conjunction(self):

        localctx = FormulaParser.ConjunctionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 108, self.RULE_conjunction)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 495
            self.constraint()
            self.state = 500
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 496
                self.match(FormulaParser.COMMA)
                self.state = 497
                self.constraint()
                self.state = 502
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ConstraintContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return FormulaParser.RULE_constraint

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class NamedTermConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def qualId(self):
            return self.getTypedRuleContext(FormulaParser.QualIdContext,0)

        def IS(self):
            return self.getToken(FormulaParser.IS, 0)
        def aggregation(self):
            return self.getTypedRuleContext(FormulaParser.AggregationContext,0)

        def funcTerm(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNamedTermConstraint" ):
                listener.enterNamedTermConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNamedTermConstraint" ):
                listener.exitNamedTermConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNamedTermConstraint" ):
                return visitor.visitNamedTermConstraint(self)
            else:
                return visitor.visitChildren(self)


    class TermConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def funcTerm(self):
            return self.getTypedRuleContext(FormulaParser.FuncTermContext,0)

        def NO(self):
            return self.getToken(FormulaParser.NO, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTermConstraint" ):
                listener.enterTermConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTermConstraint" ):
                listener.exitTermConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTermConstraint" ):
                return visitor.visitTermConstraint(self)
            else:
                return visitor.visitChildren(self)


    class SetEmptyConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def NO(self):
            return self.getToken(FormulaParser.NO, 0)
        def setComprehension(self):
            return self.getTypedRuleContext(FormulaParser.SetComprehensionContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSetEmptyConstraint" ):
                listener.enterSetEmptyConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSetEmptyConstraint" ):
                listener.exitSetEmptyConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSetEmptyConstraint" ):
                return visitor.visitSetEmptyConstraint(self)
            else:
                return visitor.visitChildren(self)


    class TypeConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def qualId(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.QualIdContext)
            else:
                return self.getTypedRuleContext(FormulaParser.QualIdContext,i)

        def IS(self):
            return self.getToken(FormulaParser.IS, 0)
        def COLON(self):
            return self.getToken(FormulaParser.COLON, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTypeConstraint" ):
                listener.enterTypeConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTypeConstraint" ):
                listener.exitTypeConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTypeConstraint" ):
                return visitor.visitTypeConstraint(self)
            else:
                return visitor.visitChildren(self)


    class BinaryArithmeticConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def arithmeticTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ArithmeticTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ArithmeticTermContext,i)

        def relOp(self):
            return self.getTypedRuleContext(FormulaParser.RelOpContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBinaryArithmeticConstraint" ):
                listener.enterBinaryArithmeticConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBinaryArithmeticConstraint" ):
                listener.exitBinaryArithmeticConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBinaryArithmeticConstraint" ):
                return visitor.visitBinaryArithmeticConstraint(self)
            else:
                return visitor.visitChildren(self)


    class DerivedConstantConstraintContext(ConstraintContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ConstraintContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def BId(self):
            return self.getToken(FormulaParser.BId, 0)
        def NO(self):
            return self.getToken(FormulaParser.NO, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDerivedConstantConstraint" ):
                listener.enterDerivedConstantConstraint(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDerivedConstantConstraint" ):
                listener.exitDerivedConstantConstraint(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDerivedConstantConstraint" ):
                return visitor.visitDerivedConstantConstraint(self)
            else:
                return visitor.visitChildren(self)



    def constraint(self):

        localctx = FormulaParser.ConstraintContext(self, self._ctx, self.state)
        self.enterRule(localctx, 110, self.RULE_constraint)
        self._la = 0 # Token type
        try:
            self.state = 527
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,51,self._ctx)
            if la_ == 1:
                localctx = FormulaParser.DerivedConstantConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 504
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==FormulaParser.NO:
                    self.state = 503
                    self.match(FormulaParser.NO)


                self.state = 506
                self.match(FormulaParser.BId)
                pass

            elif la_ == 2:
                localctx = FormulaParser.TermConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 508
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==FormulaParser.NO:
                    self.state = 507
                    self.match(FormulaParser.NO)


                self.state = 510
                self.funcTerm()
                pass

            elif la_ == 3:
                localctx = FormulaParser.SetEmptyConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 511
                self.match(FormulaParser.NO)
                self.state = 512
                self.setComprehension()
                pass

            elif la_ == 4:
                localctx = FormulaParser.TypeConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 4)
                self.state = 513
                self.qualId()
                self.state = 514
                _la = self._input.LA(1)
                if not(_la==FormulaParser.IS or _la==FormulaParser.COLON):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 515
                self.qualId()
                pass

            elif la_ == 5:
                localctx = FormulaParser.NamedTermConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 5)
                self.state = 517
                self.qualId()
                self.state = 518
                self.match(FormulaParser.IS)
                self.state = 521
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,50,self._ctx)
                if la_ == 1:
                    self.state = 519
                    self.aggregation()
                    pass

                elif la_ == 2:
                    self.state = 520
                    self.funcTerm()
                    pass


                pass

            elif la_ == 6:
                localctx = FormulaParser.BinaryArithmeticConstraintContext(self, localctx)
                self.enterOuterAlt(localctx, 6)
                self.state = 523
                self.arithmeticTerm(0)
                self.state = 524
                self.relOp()
                self.state = 525
                self.arithmeticTerm(0)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FuncTermContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def qualId(self):
            return self.getTypedRuleContext(FormulaParser.QualIdContext,0)


        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)

        def funcTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.FuncTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.FuncTermContext,i)


        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def atom(self):
            return self.getTypedRuleContext(FormulaParser.AtomContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_funcTerm

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFuncTerm" ):
                listener.enterFuncTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFuncTerm" ):
                listener.exitFuncTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFuncTerm" ):
                return visitor.visitFuncTerm(self)
            else:
                return visitor.visitChildren(self)




    def funcTerm(self):

        localctx = FormulaParser.FuncTermContext(self, self._ctx, self.state)
        self.enterRule(localctx, 112, self.RULE_funcTerm)
        self._la = 0 # Token type
        try:
            self.state = 542
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,53,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 529
                self.qualId()
                self.state = 530
                self.match(FormulaParser.LPAREN)
                self.state = 531
                self.funcTerm()
                self.state = 536
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while _la==FormulaParser.COMMA:
                    self.state = 532
                    self.match(FormulaParser.COMMA)
                    self.state = 533
                    self.funcTerm()
                    self.state = 538
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)

                self.state = 539
                self.match(FormulaParser.RPAREN)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 541
                self.atom()
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FuncTermListContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def funcTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.FuncTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.FuncTermContext,i)


        def COMMA(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.COMMA)
            else:
                return self.getToken(FormulaParser.COMMA, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_funcTermList

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFuncTermList" ):
                listener.enterFuncTermList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFuncTermList" ):
                listener.exitFuncTermList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFuncTermList" ):
                return visitor.visitFuncTermList(self)
            else:
                return visitor.visitChildren(self)




    def funcTermList(self):

        localctx = FormulaParser.FuncTermListContext(self, self._ctx, self.state)
        self.enterRule(localctx, 114, self.RULE_funcTermList)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 544
            self.funcTerm()
            self.state = 549
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while _la==FormulaParser.COMMA:
                self.state = 545
                self.match(FormulaParser.COMMA)
                self.state = 546
                self.funcTerm()
                self.state = 551
                self._errHandler.sync(self)
                _la = self._input.LA(1)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ArithmeticTermContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return FormulaParser.RULE_arithmeticTerm

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)


    class BaseArithTermContext(ArithmeticTermContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ArithmeticTermContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def atom(self):
            return self.getTypedRuleContext(FormulaParser.AtomContext,0)

        def aggregation(self):
            return self.getTypedRuleContext(FormulaParser.AggregationContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBaseArithTerm" ):
                listener.enterBaseArithTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBaseArithTerm" ):
                listener.exitBaseArithTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBaseArithTerm" ):
                return visitor.visitBaseArithTerm(self)
            else:
                return visitor.visitChildren(self)


    class AddSubArithTermContext(ArithmeticTermContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ArithmeticTermContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def arithmeticTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ArithmeticTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ArithmeticTermContext,i)

        def PLUS(self):
            return self.getToken(FormulaParser.PLUS, 0)
        def MINUS(self):
            return self.getToken(FormulaParser.MINUS, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAddSubArithTerm" ):
                listener.enterAddSubArithTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAddSubArithTerm" ):
                listener.exitAddSubArithTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAddSubArithTerm" ):
                return visitor.visitAddSubArithTerm(self)
            else:
                return visitor.visitChildren(self)


    class ParenWrappedArithTermContext(ArithmeticTermContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ArithmeticTermContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def LPAREN(self):
            return self.getToken(FormulaParser.LPAREN, 0)
        def arithmeticTerm(self):
            return self.getTypedRuleContext(FormulaParser.ArithmeticTermContext,0)

        def RPAREN(self):
            return self.getToken(FormulaParser.RPAREN, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterParenWrappedArithTerm" ):
                listener.enterParenWrappedArithTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitParenWrappedArithTerm" ):
                listener.exitParenWrappedArithTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParenWrappedArithTerm" ):
                return visitor.visitParenWrappedArithTerm(self)
            else:
                return visitor.visitChildren(self)


    class ModArithTermContext(ArithmeticTermContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ArithmeticTermContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def arithmeticTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ArithmeticTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ArithmeticTermContext,i)

        def MOD(self):
            return self.getToken(FormulaParser.MOD, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterModArithTerm" ):
                listener.enterModArithTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitModArithTerm" ):
                listener.exitModArithTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitModArithTerm" ):
                return visitor.visitModArithTerm(self)
            else:
                return visitor.visitChildren(self)


    class MulDivArithTermContext(ArithmeticTermContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a FormulaParser.ArithmeticTermContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def arithmeticTerm(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(FormulaParser.ArithmeticTermContext)
            else:
                return self.getTypedRuleContext(FormulaParser.ArithmeticTermContext,i)

        def MUL(self):
            return self.getToken(FormulaParser.MUL, 0)
        def DIV(self):
            return self.getToken(FormulaParser.DIV, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterMulDivArithTerm" ):
                listener.enterMulDivArithTerm(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitMulDivArithTerm" ):
                listener.exitMulDivArithTerm(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitMulDivArithTerm" ):
                return visitor.visitMulDivArithTerm(self)
            else:
                return visitor.visitChildren(self)



    def arithmeticTerm(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = FormulaParser.ArithmeticTermContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 116
        self.enterRecursionRule(localctx, 116, self.RULE_arithmeticTerm, _p)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 561
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [FormulaParser.LPAREN]:
                localctx = FormulaParser.ParenWrappedArithTermContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx

                self.state = 553
                self.match(FormulaParser.LPAREN)
                self.state = 554
                self.arithmeticTerm(0)
                self.state = 555
                self.match(FormulaParser.RPAREN)
                pass
            elif token in [FormulaParser.BId, FormulaParser.DECIMAL, FormulaParser.REAL, FormulaParser.FRAC, FormulaParser.STRING]:
                localctx = FormulaParser.BaseArithTermContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 559
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,55,self._ctx)
                if la_ == 1:
                    self.state = 557
                    self.atom()
                    pass

                elif la_ == 2:
                    self.state = 558
                    self.aggregation()
                    pass


                pass
            else:
                raise NoViableAltException(self)

            self._ctx.stop = self._input.LT(-1)
            self.state = 574
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,58,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    self.state = 572
                    self._errHandler.sync(self)
                    la_ = self._interp.adaptivePredict(self._input,57,self._ctx)
                    if la_ == 1:
                        localctx = FormulaParser.MulDivArithTermContext(self, FormulaParser.ArithmeticTermContext(self, _parentctx, _parentState))
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_arithmeticTerm)
                        self.state = 563
                        if not self.precpred(self._ctx, 4):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 4)")
                        self.state = 564
                        _la = self._input.LA(1)
                        if not(_la==FormulaParser.MUL or _la==FormulaParser.DIV):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 565
                        self.arithmeticTerm(5)
                        pass

                    elif la_ == 2:
                        localctx = FormulaParser.ModArithTermContext(self, FormulaParser.ArithmeticTermContext(self, _parentctx, _parentState))
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_arithmeticTerm)
                        self.state = 566
                        if not self.precpred(self._ctx, 3):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 3)")
                        self.state = 567
                        self.match(FormulaParser.MOD)
                        self.state = 568
                        self.arithmeticTerm(4)
                        pass

                    elif la_ == 3:
                        localctx = FormulaParser.AddSubArithTermContext(self, FormulaParser.ArithmeticTermContext(self, _parentctx, _parentState))
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_arithmeticTerm)
                        self.state = 569
                        if not self.precpred(self._ctx, 2):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 2)")
                        self.state = 570
                        _la = self._input.LA(1)
                        if not(_la==FormulaParser.PLUS or _la==FormulaParser.MINUS):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 571
                        self.arithmeticTerm(3)
                        pass

             
                self.state = 576
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,58,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx


    class AtomContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def qualId(self):
            return self.getTypedRuleContext(FormulaParser.QualIdContext,0)


        def constant(self):
            return self.getTypedRuleContext(FormulaParser.ConstantContext,0)


        def getRuleIndex(self):
            return FormulaParser.RULE_atom

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAtom" ):
                listener.enterAtom(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAtom" ):
                listener.exitAtom(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAtom" ):
                return visitor.visitAtom(self)
            else:
                return visitor.visitChildren(self)




    def atom(self):

        localctx = FormulaParser.AtomContext(self, self._ctx, self.state)
        self.enterRule(localctx, 118, self.RULE_atom)
        try:
            self.state = 579
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [FormulaParser.BId]:
                self.enterOuterAlt(localctx, 1)
                self.state = 577
                self.qualId()
                pass
            elif token in [FormulaParser.DECIMAL, FormulaParser.REAL, FormulaParser.FRAC, FormulaParser.STRING]:
                self.enterOuterAlt(localctx, 2)
                self.state = 578
                self.constant()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class QualIdContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def BId(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.BId)
            else:
                return self.getToken(FormulaParser.BId, i)

        def DOT(self, i:int=None):
            if i is None:
                return self.getTokens(FormulaParser.DOT)
            else:
                return self.getToken(FormulaParser.DOT, i)

        def getRuleIndex(self):
            return FormulaParser.RULE_qualId

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterQualId" ):
                listener.enterQualId(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitQualId" ):
                listener.exitQualId(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitQualId" ):
                return visitor.visitQualId(self)
            else:
                return visitor.visitChildren(self)




    def qualId(self):

        localctx = FormulaParser.QualIdContext(self, self._ctx, self.state)
        self.enterRule(localctx, 120, self.RULE_qualId)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 581
            self.match(FormulaParser.BId)
            self.state = 586
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,60,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    self.state = 582
                    self.match(FormulaParser.DOT)
                    self.state = 583
                    self.match(FormulaParser.BId) 
                self.state = 588
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,60,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ConstantContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def DECIMAL(self):
            return self.getToken(FormulaParser.DECIMAL, 0)

        def REAL(self):
            return self.getToken(FormulaParser.REAL, 0)

        def FRAC(self):
            return self.getToken(FormulaParser.FRAC, 0)

        def STRING(self):
            return self.getToken(FormulaParser.STRING, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_constant

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterConstant" ):
                listener.enterConstant(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitConstant" ):
                listener.exitConstant(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitConstant" ):
                return visitor.visitConstant(self)
            else:
                return visitor.visitChildren(self)




    def constant(self):

        localctx = FormulaParser.ConstantContext(self, self._ctx, self.state)
        self.enterRule(localctx, 122, self.RULE_constant)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 589
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.DECIMAL) | (1 << FormulaParser.REAL) | (1 << FormulaParser.FRAC) | (1 << FormulaParser.STRING))) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class BinOpContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def MUL(self):
            return self.getToken(FormulaParser.MUL, 0)

        def DIV(self):
            return self.getToken(FormulaParser.DIV, 0)

        def MOD(self):
            return self.getToken(FormulaParser.MOD, 0)

        def PLUS(self):
            return self.getToken(FormulaParser.PLUS, 0)

        def MINUS(self):
            return self.getToken(FormulaParser.MINUS, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_binOp

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBinOp" ):
                listener.enterBinOp(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBinOp" ):
                listener.exitBinOp(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBinOp" ):
                return visitor.visitBinOp(self)
            else:
                return visitor.visitChildren(self)




    def binOp(self):

        localctx = FormulaParser.BinOpContext(self, self._ctx, self.state)
        self.enterRule(localctx, 124, self.RULE_binOp)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 591
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.PLUS) | (1 << FormulaParser.MINUS) | (1 << FormulaParser.MUL) | (1 << FormulaParser.DIV) | (1 << FormulaParser.MOD))) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class RelOpContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def EQ(self):
            return self.getToken(FormulaParser.EQ, 0)

        def NE(self):
            return self.getToken(FormulaParser.NE, 0)

        def LT(self):
            return self.getToken(FormulaParser.LT, 0)

        def LE(self):
            return self.getToken(FormulaParser.LE, 0)

        def GT(self):
            return self.getToken(FormulaParser.GT, 0)

        def GE(self):
            return self.getToken(FormulaParser.GE, 0)

        def COLON(self):
            return self.getToken(FormulaParser.COLON, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_relOp

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterRelOp" ):
                listener.enterRelOp(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitRelOp" ):
                listener.exitRelOp(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitRelOp" ):
                return visitor.visitRelOp(self)
            else:
                return visitor.visitChildren(self)




    def relOp(self):

        localctx = FormulaParser.RelOpContext(self, self._ctx, self.state)
        self.enterRule(localctx, 126, self.RULE_relOp)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 593
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & ((1 << FormulaParser.COLON) | (1 << FormulaParser.EQ) | (1 << FormulaParser.NE) | (1 << FormulaParser.LE) | (1 << FormulaParser.GE) | (1 << FormulaParser.LT) | (1 << FormulaParser.GT))) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class FunModifierContext(ParserRuleContext):

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def WEAKARROW(self):
            return self.getToken(FormulaParser.WEAKARROW, 0)

        def STRONGARROW(self):
            return self.getToken(FormulaParser.STRONGARROW, 0)

        def getRuleIndex(self):
            return FormulaParser.RULE_funModifier

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFunModifier" ):
                listener.enterFunModifier(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFunModifier" ):
                listener.exitFunModifier(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFunModifier" ):
                return visitor.visitFunModifier(self)
            else:
                return visitor.visitChildren(self)




    def funModifier(self):

        localctx = FormulaParser.FunModifierContext(self, self._ctx, self.state)
        self.enterRule(localctx, 128, self.RULE_funModifier)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 595
            _la = self._input.LA(1)
            if not(_la==FormulaParser.STRONGARROW or _la==FormulaParser.WEAKARROW):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx



    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates == None:
            self._predicates = dict()
        self._predicates[58] = self.arithmeticTerm_sempred
        pred = self._predicates.get(ruleIndex, None)
        if pred is None:
            raise Exception("No predicate with index:" + str(ruleIndex))
        else:
            return pred(localctx, predIndex)

    def arithmeticTerm_sempred(self, localctx:ArithmeticTermContext, predIndex:int):
            if predIndex == 0:
                return self.precpred(self._ctx, 4)
         

            if predIndex == 1:
                return self.precpred(self._ctx, 3)
         

            if predIndex == 2:
                return self.precpred(self._ctx, 2)
         




