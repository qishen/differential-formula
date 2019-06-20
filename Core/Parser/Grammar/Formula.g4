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
	: typeDecl # DomTypeExpr
	| formulaRule # DomRuleExpr
	| CONFORMS funcTermList # DomConformsExpr
	;

/**************** Type Decls *****************/

typeDecl
	: Id TYPEDEF (funcDecl)? LPAREN fields RPAREN # RegularTypeDecl
	| Id TYPEDEF unnBody # UnionTypeDecl
	;

unnBody
	: unnElem (PLUS unnElem)* ;

funcDecl : INJ | BIJ | SUR | FUN | SUB | NEW;

fields
	: field (COMMA field)* ;

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
	: funcTerm (COMMA funcTerm)* DOT 
	;

formulaRule 
	: funcTermList      
	  DOT                
	| funcTermList
	  RULE               
	  disjunction
	  DOT				
	;

/* 
Set comprehension is the form of {t1, t2,... tn | body}, t1...tn can be a combination
of constants, variables and predicates.
For every assignment that satisfies the body, substitute values to each t_i and add t_i
to set S.
The body of set comprehension can have nested comprehension inside it.
For example, count({a, b | a is V, E(a, a'), count({a' | Element(a', b) }) < 2 }) = 1. It does
not make sense but fully supported in FORMULA.
*/
setComprehension
	: LBRACE funcTermList RBRACE
	| LBRACE funcTermList PIPE disjunction RBRACE
	;

disjunction 
	: conjunction			  
	| conjunction SEMICOLON disjunction
	;

conjunction 
	: constraint          
	| constraint COMMA conjunction
	;

/******************* Terms and Constraints *******************/

/*
Five kinds of constraints in FORMULA rules or queries.
1. Existence or absence of compositional term. e.g. no A(a, b).
2. Aggregation over set comprehension.
Binary constraint are matched only for arithmetic terms over binary relation
and only limited numeric types.
*/
constraint
	: (NO)? compositionalTermWithoutAlias # PredConstraint
	| (NO)? setComprehension # ComprConstraint
	| arithmeticTerm relOp arithmeticTerm # BinaryConstraint
	| Id IS Id # TypeConstraint
	| Id IS compositionalTermWithoutAlias # VariableConstraint
	;

compositionalTermList
	: compositionalTerm (COMMA compositionalTerm)*
	;

funcTermList 
	: funcTerm (COMMA funcTerm)* 
	;

funcTerm 
	: compositionalTerm 
    | compositionalTermWithoutAlias
	| arithmeticTerm
	;

compositionalTerm 
	: (Id (IS | EQ))? Id LPAREN funcTermList RPAREN
	;

compositionalTermWithoutAlias
	: Id LPAREN funcTermList RPAREN
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

unOp : MINUS ;

binOp : MUL | DIV | MOD | PLUS | MINUS ;

relOp : EQ | NE | LT | LE | GT | GE | COLON ;

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