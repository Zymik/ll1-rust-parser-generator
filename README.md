# ll1-rust-parser-generator
LL1 grammar parser generator in Rust. It's a partly analogue of ANTLR that generates code for parsing in Rust. Generator supports synthesized and inherited attributes. You can set tokens for lexical analysis and characters to skip as regex.
Grammar for calculator with +,-,*,/ and combinations example:


```
Prelude {
    fn fact(n: i64) -> i64 {
        if n < 0 {
            panic!("Factorial less than zero");
        }
        let mut s: i64 = 1;
        for i in 1..n + 1 {
            s *= i;
        }
        s
    }

    fn comb(n: i64, k: i64) -> i64 {
        let n_fact = fact(n);
        let k_fact = fact(k);
        let n_k_fact = fact(n - k);
        n_fact / (k_fact * n_k_fact)
    }

}

Skip {
    "\n";
    "\r";
    " ";
}

Tokens {
    Num -> "(-?)[1-9]([0-9]*)";
    Mul -> "\\*";
    Plus -> "\\+";
    Minus -> "\\-";
    Div -> "/";
    Comb -> "\\$";
    Lb -> "\\(";
    Rb -> "\\)";
}

NotTerminals {

    S {} {res # i64} -> C {res = C0_res;};

    C {} {res # i64} -> E Cx (E0_res) {res = Cx1_res} ;

    Cx {acc # i64} {res # i64} -> Comb E Cx(comb(acc, E1_res)) {res = Cx2_res} | {res = acc;};

    E {} {res # i64} -> T Ex (T0_res) {res = Ex1_res;};

    Ex {acc # i64} {res # i64} -> Plus T Ex(acc + T1_res) {res = Ex2_res;} |
                                  Minus T Ex(acc - T1_res) {res = Ex2_res;} |
                                  {res = acc;};

    T {} {res # i64} -> F Tx(F0_res) {res = Tx1_res;};

    Tx {acc # i64} {res # i64} -> Mul F Tx(acc * F1_res) {res = Tx2_res;} |
                                  Div F Tx(acc / F1_res) {res = Tx2_res;} |
                                  {res = acc;};

    F {} {res # i64} -> Minus F {res = - F1_res;} | P {res = P0_res;};

    P {} {res # i64} -> Lb E Rb {res = E1_res;} | Num {res = Num0_ident.parse().unwrap();};

}
```
Parser Description format:
 1. 'Prelude' in the beginning will prepend to Parser source code. 
 It's a place where you should describe imports, const and our functions
 2. 'Skip' describes skippable letters. It's a list of regexes, separated by ';'
 3. 'Tokens' describes tokens in which passed to parser strings will be tokenized. It's list of `Token_Name -> "Regex"` separated by ';'
 4. NotTerminals describes grammar rules. Each rules for one not terminal have format 
    `NAME {(input_arg # type)*} {(return_arg # type)} -> ((Token | NotTerminal \(input_args\)| {Rust Code})\|)*;`
    You can get access to return args of Token or NotTerminal by position in list of one rule like this `F1_res`

