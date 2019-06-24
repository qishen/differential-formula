grammar Formula;

/*
 * Parser Rules
 */

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
	: Id
	| Id AT STRING
	| Id RENAMES Id
	| Id RENAMES Id AT STRING
	;

model 
	: modelSig LBRACE (modelFactList)* RBRACE
	;

modelIntro
	: (PARTIAL)? MODEL Id OF modRef
	;

modelSig
	: modelIntro ((INCLUDES | EXTENDS) modRefs)?
	;

domain 
	: domainSig LBRACE domSentences RBRACE
	;

domainSig
	: DOMAIN Id ((EXTENDS | INCLUDES) modRefs)? 
	;

domSentences
	: (domSentence DOT)*
	;

domSentence
	: typeDecl # DomTypeSentence
	| formulaRule # DomRuleSentence
	| CONFORMS disjunction # DomConformsSentence
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


/************* Facts, Rules, and Comprehensions **************/
modelFactList
	: modelFact (COMMA modelFact)* DOT 
	;

modelFact
	: compositionalTerm
	| compositionalTermWithoutAlias
	;

formulaRule 
	: terms RULE disjunction DOT				
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
	: LBRACE terms (PIPE conjunction)? RBRACE
	;

/*
Disjunction of conjunction of constraints.
*/
disjunction 
	: conjunction (SEMICOLON conjunction)
	;

/*
Conjunction of constraints.
*/
conjunction 
	: constraint (COMMA constraint)
	;

/******************* Terms and Constraints *******************/

/*
TODO: Is it possible to have variable of integer type to compare aggregation result.

Seven kinds of constraints in FORMULA rules or queries.
1. Existence or absence of compositional term. e.g. no A(a, b).
2. Aggregation over set comprehension and compare with numeric value. count({x|...}) = 1.
3. Binary relation over arithmetic terms.
4. Type constraint for variables
5. Variable constraints binding to predicates with other variables as arguments.
Binary constraint are matched only for arithmetic terms over binary relation
6. Derived constant x is provable
7. true if set comprehension is empty otherwise false.
and only limited numeric types.
*/
constraint
	: (NO)? compositionalTermWithoutAlias # PredConstraint
	| (NO)? COUNT LPAREN setComprehension RPAREN relOp DECIMAL # AggregationCountConstraint
	| arithmeticTerm relOp arithmeticTerm # BinaryConstraint
	| Id (IS | EQ) Id # TypeConstraint
	| Id (IS | EQ) compositionalTermWithoutAlias # VariableBindingConstraint
	| (NO)? Id # DerivedConstantConstraint
	| NO setComprehension # SetComprehensionConstraint
	;

/*
A term can be variable, constant, arithmetic expression or 
compositional term consists of variable, constant or itself as arguments.
Arithmetic term can be viewed as compositional term as well in the form of <op>(a, b). 
term = "hello"
term = x
term = Edge(x, y)
term = Edge(Node(x+2), Node(y*3))

Note that arithmetic term may not be allowed in model fact but allowed in rule head.

*/
term
	: compositionalTermWithoutAlias | arithmeticTerm | atom
	;

/*
terms can be used in the head of FORMULA rule and its head
should only contain boolean variables or compositional terms with
some variables inside it.
*/
terms 
	: term (COMMA term)*
	;

compositionalTerm 
	: Id (IS | EQ) Id LPAREN terms RPAREN
	;

compositionalTermWithoutAlias
	: Id LPAREN terms RPAREN
	;

/*
Operator precedence (* or /) -> MOD -> (+ or -) and no right associativity is needed.
The basic arithmetic term is either variable or constant.
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