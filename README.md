### Json generator
The simple console utility to generate JSON items according to the provided example composing JSON body 
and a set of functions that define the logic to generate new items.
 
The utility allows delivering the generated JSON to different sources such as an HTTP server, folder or file or console

### Overall

Given template:
```json
{
  "description": "the example how to create a json template to generate new jsons",
  "note": "the prefix | in a field name signals that the field carries a function to generate data.",
  "record": {
    "|type": "str_from_list(business,technical,analytical)", 
    "technical":{
       "|id": "uuid()",
       "|index": "seq()",
       "|updated_tm": "dt()",
       "|created_tm": "dt(%Y-%m-%d %H:%M:%S)"
     },
    "|is_active": "bool()",
    "|name": "str(10,customer)",
    "|email": "str(5,,@gmail.com)",
    "|code": "str(5,,'(code)')",
    "|dsc": "str(20)",
    "geo": {
        "|country": "str_from_file(jsons/countries,,)",
        "|city": "str_from_file(jsons/cities,'\n')",
        "|street": "str(10,,-street)",
        "|house": "int(1,1000)"
      },
    "|id_parent": "int_from_list(1,2,3,4,5,6,7)",
    "|related_records": "int(1,1000) -> array(5)"

  }
}
```

Generated json:
```json
{
  "description": "the example how to create a json template to generate new jsons",
  "note": "the prefix | in a field name signals that the field carries a function to generate data.",
  "record": {
    "code": "Upaz2(code)",
    "dsc": "gLgvDinPZg1aMu9LpPyp",
    "email": "PMWtc@gmail.com",
    "geo": {
      "city": "Rome",
      "country": "Australia",
      "house": 770,
      "street": "7Ke4CAHWpk-street"
    },
    "id_parent": 7,
    "is_active": false,
    "name": "customerXgKChm5t2b",
    "related_records": [
      263,
      489,
      390,
      226,
      361
    ],
    "technical": {
      "created_tm": "2021-05-23 13:09:27",
      "id": "339b0ca7-0e00-4d6e-8073-d270d7d56e2e",
      "index": 1,
      "updated_tm": "2021-05-23 13:09:27"
    },
    "type": "analytical"
  }
}
```

### Generated rules
Overall, if the field does not have a specific prefix, depicting that the field carries a generator function, 
the value of the field will be taken and returned in the result.
Otherwise, if the field contains a prefix in its name the value is expected to be a string and describe the function to generate the values.
By default the prefix is ```|``` like in the example:
```json
{
  "|dynamic_field_with_generator": "seq()",
  "static_field": "constant"
}
``` 

*Note: the prefix sign can be changed if it interferes with the existing field into any other char.

### Generators
Every generator has a following syntax:
``` generator name ( arg1, arg2, ..) ```

If the generator contains another generator then the syntax obtain extra elements:
``` internal generator (arg1, arg2, .. ) -> encompassing generator (arg1, arg2, ..) ```

The generators can have empty arguments:
``` str(10,,postfix)```

The string literals can be placed as an argument straightly or encompassed by the single quotes:
``` str(10,literal,'with quotes')```
 
#### List of generators:
| Generator | Arguments=default value | Description | Example |
|----------------------|--------------------------------------------|----------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| seq | starting point=0,step=1 | the sequentially-increase row of numbers (1,2,3,4 ...) | seq() / seq(10,2) / seq(,2)  |
| bool |  | generated boolean  | bool() |
| str | size of row=0,prefix='',suffix='' | the row composed of random letters and numbers, predefined length having prefix and suffix | str() / str(10) / str(,prefix,) / str(,,'suffix') / str(10,abc,cde)|
| int | low bound=0 and high bound=1000 | the random number lying in predefined bounds | int() / int(1,100) / int(1) / int(,10) |
| str_from_list | list of values | the list of string | str_from_list(a,'b',c,d) |
| int_from_list | list of values | list of numbers | int_from_list(1,2,3,4,5) |
| str_from_file | path to file, delimiter=','  | the list of string pulled off the predefined file note: delimiter can be omitted and the default delimiter(,) will be used | str_from_file(\home\user\json) str_from_file(\home\user\json,;)  str_from_file(\home\user\json,\n) |
| int_from_file | path to file, delimiter=','  | list of numbers pulled off the predefined file note: delimiter can be omitted and the default delimiter(,) will be used  |  int_from_file(c:\\user\json)  |
| uuid |  | generated uuid  | uuid() |'
| dt | format=%Y-%m-%d %H:%M:%S | the current date and time. | dt(%Y-%m-%d)/dt() |
| array | func_to_generate -> array(number=1) | the generator to get the array filled. | int(1) -> array() |


### How to use

### From console
#### Command line example

```bush
cargo install json-gen
```

```bash
json-gen  -f "file path" -r 10 --pretty --logs --to-folder folder --to-curl '-X POST ip'
```    

##### Command line Arguments
| Short | Long  | Description                                                                                                 | Example                                                               |
|----------|-----------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------|
| b        | body | the text represents the json template                                                                          | --body \| -b '{"k":"v"}'                                         |
| f        | file | the path to file including the json template                                                                        | --file \| -f c:\\folder\json.json                                |
| r        | repeat    | the number of repetitions                                                                      | --repeat \| -r 10                                                     |
| i        | indicator    | the indicator signalling the field carries the function to generate.                                                                      | --indicator \| -i >                                                     |
|          | pretty    | inserts formatting symbols to get json readable                                                           | --pretty                                                              |
|          | logs     | prints logs                                                                                                 | --logs                                                               |
|          | to-console    | show json in console(by default if outputs array is empty)                                                  | --to-console                                                             |
|          | to-file   | append generated jsons to file. If the file does not exist.  it creates a new one. The folder should exist. | --to-file c:\\folder\jsons.json                                       |
|          | to-folder | creates new files and place it to the selected folder.  It creates folder if it not exists.                 | --to-file c:\\folder                                                  |
|          | to-curl   | sends jsons to the server using curl for that. In fact,  the -d will be added.                              | --to-curl "-H Content-Type:application/json -X POST 127.0.0.1:7878" |
| h       | help    | information  about commands                                                                                  | -h \| --help                                                          |
| V       | version | version                                                                                                     | -V \| --version                                                       |
 
**note**: for using --to-curl  parameter the system needs to have the curl utility installed.

### From dependency

```toml
json-gen="*"
```

```rust
use json_generator::json_template::JsonTemplate;
use json_generator::generate;
use serde_json::Value;

fn main() {
    let json_template:&str = "{\"|id\":\"seq()\"}";
    let mut json_template = JsonTemplate::from_str(json_template, "|").unwrap();
    let generated_value:Vec<Value> = generate(&mut json_template,10,true,&mut vec![]);
}
```

#### Senders
The function generate gets the last parameter it is an array of senders.
Essentially, sender is a struct implementing a trait sender:
```rust
pub trait Sender {
    fn send(&mut self, json: &Value, pretty: bool) -> Result<String, GenError>;
}
```
and example of a simple implementation:

```rust
 use json_generator::sender::{Sender, ConsoleSender, string_from};
 use serde_json::Value;
 
impl Sender for ConsoleSender {
fn send(&mut self, json: &Value, pretty: bool) -> Result<String, GenError> {
    println!("{}",  string_from(json, pretty)?);
    Ok("the item has been sent to the console".to_string())
   }
 }

```

#### GenError
By default, everything is wrapped with Gen(erator)Error, the general structure to handle errors.

In general, it has the following bowels:

```rust
#[derive(Debug)]
pub struct GenError {
    reason: String,
    tpe: GenErrorType,
}

#[derive(Debug)]
pub enum GenErrorType {
    Parser,
    Sender,
    Generator,
    Common,
}
```

and provides the following methods to work with:
```rust
impl GenError {
    pub fn new_with(reason: &str) -> Self {..}
    pub fn new_with_in_parser(reason: &str) -> Self {..}
    pub fn new_with_in_sender(reason: &str) -> Self {..}
    pub fn new_with_in_generator(reason: &str) -> Self {..}
}
```