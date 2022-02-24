var searchIndex = JSON.parse('{\
"hyperloglog":{"doc":"A hyperloglog implementation in Rust.","t":[13,3,18,18,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,4,13,3,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Hasher","HyperLogLog","MAX","MIN","P10","P11","P12","P13","P14","P15","P16","P17","P18","P4","P5","P6","P7","P8","P9","Precision","Precision","TryFromIntError","TryMergeError","TryMergeErrorKind","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clear","clone","clone","clone","clone","clone","clone_from","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","default","default","eq","eq","eq","eq","extend","extend","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from_iter","get","hash","hasher","in_range","insert","insert_hash","into","into","into","into","into","is_empty","kind","len","merge_from_unchecked","merge_unchecked","ne","ne","new","new","new","partial_cmp","precision","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_merge","try_merge_from","type_id","type_id","type_id","type_id","type_id","variants","with_hasher","with_precision","with_precision_and_hasher"],"q":["hyperloglog","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Hyperloglogs use different hashers.","A hyperloglog data structure to estimate the number of …","The largest precision: 18 bits.","The smallest precision: 4 bits.","10-bit precision.","11-bit precision.","12-bit precision (default).","13-bit precision.","14-bit precision.","15-bit precision.","16-bit precision.","17-bit precision.","18-bit precision.","4-bit precision.","5-bit precision.","6-bit precision.","7-bit precision.","8-bit precision.","9-bit precision.","The precision of the hyperloglog, that is, the number of …","Hyperloglogs have different precisions.","Error type returned when converting an integer into a …","Error type returned when a merge operation fails.","A list specifying categories of merging error.","","","","","","","","","","","Clears the hyperloglog, removing all values.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Return the precision value as an <code>u8</code>.","","Returns a reference to the hyperloglog’s <code>BuildHasher</code>.","Checks whether the given value is in the range of valid …","Adds a value to the hyperloglog.","Adds a hash value to the hyperloglog.","","","","","","Returns <code>true</code> if the hyperloglog contains no elements.","Returns the corresponding <code>TryMergeErrorKind</code> for this error.","Calculates the approximate number of different elements.","Merges the hyperloglog <code>rhs</code> into <code>self</code> without checking that …","Merges two hyperloglogs without checking that precisions …","","","Creates a new empty hyperloglog with the default precision.","Creates a new merge error from a known kind of error.","Creates a precision if the given value is in range.","","Returns the precision of the hyperloglog.","","","","","","","","","","","","","","","","","","","","Merges two hyperloglogs.","Merges the hyperloglog <code>rhs</code> into <code>self</code>.","","","","","","All possible precisions, in increasing order.","Creates a new empty hyperloglog with the default precision …","Creates a new empty hyperloglog with the given precision.","Creates a new empty hyperloglog with the given precision …"],"i":[1,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,1,0,0,0,3,4,1,5,2,3,4,1,5,2,3,3,4,1,5,2,3,3,4,1,5,2,2,3,2,4,1,5,2,3,3,3,4,4,1,5,5,2,2,3,3,4,1,5,5,2,3,2,2,3,2,3,3,3,4,1,5,2,3,5,3,3,3,4,5,3,5,2,2,3,3,4,1,5,2,4,5,2,3,4,1,5,2,2,3,4,1,5,2,3,3,3,4,1,5,2,2,3,3,3],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["tryfrominterror",3]],[[],["trymergeerrorkind",4]],[[],["trymergeerror",3]],[[],["precision",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[["precision",4]],["ordering",4]],[[]],[[]],[[["tryfrominterror",3]],["bool",15]],[[["trymergeerrorkind",4]],["bool",15]],[[["trymergeerror",3]],["bool",15]],[[["precision",4]],["bool",15]],[[["intoiterator",8]]],[[["intoiterator",8]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[["trymergeerrorkind",4]]],[[]],[[]],[[["intoiterator",8]]],[[],["u8",15]],[[]],[[]],[[["u8",15]],["bool",15]],[[]],[[["u64",15]]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["trymergeerrorkind",4]],[[],["usize",15]],[[]],[[]],[[["tryfrominterror",3]],["bool",15]],[[["trymergeerror",3]],["bool",15]],[[]],[[["trymergeerrorkind",4]]],[[["u8",15]],["option",4]],[[["precision",4]],["option",4,[["ordering",4]]]],[[],["precision",4]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["u8",15]],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4,[["trymergeerror",3]]]],[[],["result",4,[["trymergeerror",3]]]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[["precision",4]]],[[["precision",4]]]],"p":[[4,"TryMergeErrorKind"],[4,"Precision"],[3,"HyperLogLog"],[3,"TryFromIntError"],[3,"TryMergeError"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};