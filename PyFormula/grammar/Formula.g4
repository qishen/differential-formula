grammar Formula;

/*
 * Parser Rules
 */

/**************** Configs *********************/
config
    : LBRACKET settingList RBRACKET
    ;

sentenceConfig
    : LBRACKET settingList RBRACKET
    ;

settingList
    : setting (COMMA setting)*
    ;

setting
    : Id EQ constant
    ;

/**************** Module Decls *****************/
program
    : EOF    
    | (importModule)? moduleList   
    ;    

importModule
	: IMPORT Id FROM STRING AS Id
	;

moduleList
    : (module)*      
    ;

module 
    : domain
	| model
    ;

modRefs
	: modRef (COMMA modRef)*
	;

modRef
	: Id (RENAMES Id)? (AT STRING)?
	;

/**************** Model Decls *****************/
model 
	: modelSigConfig LBRACE (modelBody)? RBRACE
	;

modelBody
    : (modelSentence)*
    ;

modelSentence
    : modelFactList
    | modelContractConf
    ;

modelContractConf
    : (sentenceConfig)? modelContract
    ;

modelContract
    : ENSURES disjunction DOT
    | REQUIRES disjunction DOT
    | REQUIRES cardSpec DECIMAL Id DOT
    ;

cardSpec
    : SOME
    | ATMOST
    | ATLEAST
    ;

modelSigConfig
    : modelSig (config)?
    ;

modelIntro
	: (PARTIAL)? MODEL Id OF modRef
	;

modelSig
	: modelIntro ((INCLUDES | EXTENDS) modRefs)?
	;

modelFactList
    : modelFact (COMMA modelFact)* DOT
    ;

modelFact
	: (Id IS)? funcTerm
    ;

/**************** Domain Decls *****************/
domain 
	: domainSigConfig LBRACE (domSentences)? RBRACE
	;

domainSigConfig
    : domainSig (config)?
    ;

domainSig
	: DOMAIN Id ((EXTENDS | INCLUDES) modRefs)? 
	;

domSentences
	: (domSentenceConfig DOT)*
	;

domSentenceConfig
    : (sentenceConfig)? domSentence
    ;

domSentence
	: typeDecl
	| formulaRule
	| CONFORMS disjunction
	;

/**************** Type Decls *****************/

typeDecl
	: Id TYPEDEF (funcDecl)? LPAREN fields RPAREN # RegularTypeDecl
	| Id TYPEDEF unnBody # UnionTypeDecl
	;

unnBody
	: unnElem (PLUS unnElem)* ;

funcDecl : INJ | BIJ | SUR | FUN | SUB | NEW;

/*
function modifier occurs only once in type definition, compiler needs
to check and throw exception if this requirement is not met.
A ::= bij(B,C,D => X,Y,Z).
*/
fields
	: field ((COMMA | funModifier) field)* ;

field
	: (Id COLON)? (ANY)? (Id | unnBody) 
	;

unnElem 
	: Id 
	| LBRACE enumList RBRACE
	;

enumList : enumCnst (COMMA enumCnst)* ;

enumCnst 
	: constant | DECIMAL RANGE DECIMAL;


/************* Constraints **************/
formulaRule 
	: funcTermList RULE disjunction
	;

/* 
Set comprehension is the form of {t1, t2,... tn | body}, t1...tn can be a combination
of constants, variables and predicates.
For every assignment that satisfies the body, substitute values to each t_i and add t_i
to set S.
The body of set comprehension can have nested comprehension inside it.
For example, count({a, b | a is V, E(a, a'), count({a' | Element(a', b) }) < 2 }) = 1. 
The example does not make sense but should be fully supported in FORMULA as a very useful
feature.
*/
setComprehension
	: LBRACE funcTermList (PIPE conjunction)? RBRACE
	;

/*
Disjunction of conjunction of constraints.
*/
disjunction 
	: conjunction (SEMICOLON conjunction)*
	;

/*
Conjunction of constraints and they are only used in the body of a rule.
*/
conjunction 
	: constraint (COMMA constraint)*
	;

/******************* Terms and Constraints *******************/

/*
TODO: Is it possible to have variable of integer type to compare aggregation result.
*/
constraint
	: (NO)? funcTerm # TermConstraint // e.g. no Edge(Node(x), Node("hello"))
	| Id (IS | EQ) funcTerm # TermConstraintWithAlias // e.g. e1 is Edge(x,y)
	| arithmeticTerm relOp arithmeticTerm # BinaryArithmeticConstraint // e.g. a + b * c > d
	| (NO)? COUNT LPAREN setComprehension RPAREN relOp DECIMAL # AggregationCountConstraint
	| NO setComprehension # SetComprehensionConstraint // e.g. no {a | a is Node}
	| (NO)? Id # DerivedConstantConstraint // e.g. no hasCycle
	| Id IS Id # TypeConstraint // e.g. n1 is Node
	;

/*
Antlr4 only supports mutually recursive definition written in one rule instead of
multiple rules. Arithmetic terms are not allowed in functerm like Node(a+1) :- Node(a) and
it should be written as Node(b) :- Node(a), b = a + 1.
*/
funcTerm
    : Id LPAREN funcTerm (COMMA funcTerm)* RPAREN
    | atom
    ;

funcTermList
    : funcTerm (COMMA funcTerm)*
    ;

/*
Operator precedence (* or /) -> MOD -> (+ or -) and no right associativity is needed.
The basic arithmetic term is either variable or constant since these operators don't
apply to instances of compositional types.
*/
arithmeticTerm
	: LPAREN arithmeticTerm RPAREN # ParenthesisArithTerm
	| arithmeticTerm (MUL | DIV) arithmeticTerm # MulDivArithTerm
	| arithmeticTerm MOD arithmeticTerm # ModArithTerm
	| arithmeticTerm (PLUS | MINUS) arithmeticTerm # AddSubArithTerm
	| atom # AtomTerm
	;

atom : Id | constant ;

constant : DECIMAL | REAL | FRAC | STRING ;

binOp : MUL | DIV | MOD | PLUS | MINUS ;

relOp : EQ | NE | LT | LE | GT | GE | COLON ;

funModifier : '->' | '=>';


/*
 * Lexer Rules
 */

/* Keywords */
DOMAIN : 'domain' ;
MODEL : 'model' ;
TRANSFORM : 'transform' ;
SYSTEM : 'system' ;

INCLUDES : 'includes' ;
EXTENDS : 'extends' ;
OF : 'of' ;
RETURNS : 'returns' ;
AT : 'at' ;
MACHINE : 'machine' ;

IS : 'is' ;
NO : 'no' ;

NEW : 'new' ;
FUN : 'fun' ;
INJ : 'inj' ;
BIJ : 'bij' ;
SUR : 'sur' ;
ANY : 'any' ;
SUB : 'sub' ;

COUNT : 'count' ;
ENSURES : 'ensures' ;
REQUIRES : 'requires' ;
CONFORMS : 'conforms' ;
SOME : 'some' ;
ATLEAST : 'atleast' ;
ATMOST : 'atmost' ;
PARTIAL : 'partial' ;
INITIALLY: 'initially' ;
NEXT : 'next' ;
PROPERTY : 'property' ;
BOOT : 'boot' ;
IMPORT : 'import';
FROM : 'from';
AS : 'as';

fragment ALPHANUMERIC: ALPHA | DIGIT ;
fragment ALPHA: '_' | SMALL_LETTER | CAPITAL_LETTER ;
fragment SMALL_LETTER: [a-z] ;
fragment CAPITAL_LETTER: [A-Z] ;
fragment DIGIT : [0-9] ;

Id : ALPHA ALPHANUMERIC* '\''* ;
DECIMAL : DIGIT+ ;
REAL : [-+]? DIGIT+ [.] DIGIT+ ;
FRAC : [-+]? DIGIT+ [/] [-+]? DIGIT* ;
STRING : '"'ALPHANUMERIC* '"';

PIPE : '|' ;
TYPEDEF : '::=' ;
RULE : ':-' ;
RENAMES : '::' ;
RANGE : '..' ;
DOT : '.' ;
COLON : ':' ;
COMMA : ',' ;
SEMICOLON : ';' ;
EQ : '=' ;
NE : '!=' ;
LE : '<=' ;
GE : '>=' ;
LT : '<' ;
GT : '>' ;
PLUS : '+' ;
MINUS : '-' ;
MUL : '*' ;
DIV : '/' ;
MOD : '%' ;
STRONGARROW : '=>' ;
WEAKARROW : '->' ;

LBRACE : '{' ;
RBRACE : '}' ;
LBRACKET : '[' ;
RBRACKET : ']' ;
LPAREN : '(' ;
RPAREN : ')' ;

WS
	: [ \t\r\n]+ -> skip ;