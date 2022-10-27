# Lambda Calculus Parser (Evaluate/Reduce)

## Grammar
>Term
>>App<br>
>>Abs<br>
>>Op<br>
>>Var<br>
>>Num<br>

>App
>>( Term Term )<br>

>Abs
>>( λ Var Term )<br>
>>( λ Vars . Term )<br>
>>λ Vars . Term<br>

>Op
>>( Term TermOp Term )<br>

>Vars
>>Var<br>
>>Var Vars<br>

>Var
>>[a-z]<br>

>Num
>>[0-9]+<br>

>TermOp
>>*<br>
>>/<br>
>>+<br>
>>-<br>
