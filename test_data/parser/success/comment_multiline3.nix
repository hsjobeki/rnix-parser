/*root*/
{
  /*attrset*/
  config = 1;

  list = [
    /*list*/
    "item1"
  ];

  /*attrset*/
  lambda = {config,...}@args: 
    /*lambda*/
    config // args;

}