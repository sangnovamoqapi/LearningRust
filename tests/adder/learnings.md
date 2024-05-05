1. For struct and enum types we need to implement the PartialEq and Debug traits, to actually use assert macros, can be done by simply annotating #[derive(PartialEq, Debug)]
