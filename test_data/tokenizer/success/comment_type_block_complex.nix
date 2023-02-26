/*
A number that configures behavior
Type: 
let 
    Derivation :: {
        all :: [ Derivation ];
        builder :: String;
        drvAttrs :: {
            builder = String; 
            name = String;
            outputs = [ output @ String ]; 
            system = String;
            ${additionalArgs} :: String;
        }
        drvPath :: String;
        name :: String;
        outPath :: String;
        outputName :: String;
        outputs :: [ output :: String ];
        system :: String;
        type :: "derivation";
        ${output} :: Derivation;
        ${additionalArgs} :: String;
    };
in
    builtins.derivation :: {
        name :: String;
        outputs :: [ output :: String ] ? [ "out" ];
        builder :: String;
        system :: String;
        ${additionalArgs} :: String;
    } -> Derivation
    
Example: 2

  - some
  - markdown

  # heading 1

*/


/*  freeText
    \nType:
    {typeSyntaxt}
    Example:
    freeText
*/

/*
    # lskljhasd

    - list


   @TYPE: 
*/



{
  foo = a: a / 2;
}