var searchIndex = JSON.parse('{\
"hyperloglog":{"doc":"A hyperloglog implementation in Rust.","t":[13,3,18,18,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,4,13,3,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Hasher","HyperLogLog","MAX","MIN","P10","P11","P12","P13","P14","P15","P16","P17","P18","P4","P5","P6","P7","P8","P9","Precision","Precision","TryFromIntError","TryMergeError","TryMergeErrorKind","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clear","clone","clone","clone","clone","clone","clone_from","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","default","default","eq","eq","eq","eq","extend","extend","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from_iter","get","hash","hasher","in_range","insert","insert_hash","into","into","into","into","into","is_empty","kind","len","merge_from_unchecked","merge_unchecked","ne","ne","new","new","partial_cmp","precision","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_merge","try_merge_from","type_id","type_id","type_id","type_id","type_id","variants","with_hasher","with_precision","with_precision_and_hasher"],"q":["hyperloglog","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Hyperloglogs use different hashers.","A hyperloglog data structure to estimate the number of …","The largest available precision: 18 bits.","The smallest available precision: 4 bits.","10-bit precision.","11-bit precision.","12-bit precision (default).","13-bit precision.","14-bit precision.","15-bit precision.","16-bit precision.","17-bit precision.","18-bit precision.","4-bit precision.","5-bit precision.","6-bit precision.","7-bit precision.","8-bit precision.","9-bit precision.","The precision of the hyperloglog, that is, the number of …","Hyperloglogs have different precisions.","Error type returned when converting an integer into a …","Error type returned when a merge operation fails.","A list specifying categories of merging error.","","","","","","","","","","","Clears the hyperloglog, removing all values.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Return the precision value as an <code>u8</code>.","","Returns a reference to the hyperloglog’s <code>BuildHasher</code>.","Checks whether the given value is in the range of valid …","Adds a value to the hyperloglog.","Adds a hash value to the hyperloglog.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if the hyperloglog contains no elements.","Returns the corresponding <code>TryMergeErrorKind</code> for this error.","Calculates the approximate number of different elements.","Merges the hyperloglog <code>rhs</code> into <code>self</code> without checking that …","Merges two hyperloglogs without checking that precisions …","","","Creates a new empty hyperloglog with the default precision.","Creates a precision if the given value is in range.","","Returns the precision of the hyperloglog.","","","","","","","","","","","","","","","","","","","","Merges two hyperloglogs.","Merges the hyperloglog <code>rhs</code> into <code>self</code>.","","","","","","All available precisions, in increasing order.","Creates a new empty hyperloglog with the default precision …","Creates a new empty hyperloglog with the given precision.","Creates a new empty hyperloglog with the given precision …"],"i":[3,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,3,0,0,0,1,2,3,4,5,1,2,3,4,5,1,1,2,3,4,5,1,1,2,3,4,5,5,1,5,2,3,4,5,1,1,1,2,2,3,4,4,5,5,1,1,2,3,4,5,1,5,5,1,5,1,1,1,2,3,4,5,1,4,1,1,1,2,4,1,5,5,1,1,2,3,4,5,2,4,5,1,2,3,4,5,5,1,2,3,4,5,1,1,1,2,3,4,5,5,1,1,1],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1],[1,1],[2,2],[3,3],[4,4],[5,5],[[1,1]],[[]],[[]],[[]],[[]],[[]],[[5,5],6],[[],1],[[],5],[[2,2],7],[[3,3],7],[[4,4],7],[[5,5],7],[[1,8]],[[1,8]],[[1,9],10],[[2,9],10],[[2,9],10],[[3,9],10],[[4,9],10],[[4,9],10],[[5,9],10],[[5,9],10],[[],[[1,[11]]]],[[]],[[]],[[]],[[]],[[]],[8,1],[5,12],[5],[1],[12,7],[1],[[1,13]],[[]],[[]],[[]],[[]],[[]],[1,7],[4,3],[1,14],[[1,1]],[[1,1],1],[[2,2],7],[[4,4],7],[[],1],[12,[[15,[5]]]],[[5,5],[[15,[6]]]],[1,5],[[]],[[]],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],17],[[],17],[[],17],[[],17],[12,[[17,[5]]]],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[1,1],[[17,[1,4]]]],[[1,1],[[17,[4]]]],[[],18],[[],18],[[],18],[[],18],[[],18],[[]],[[],1],[5,1],[5,1]],"p":[[3,"HyperLogLog"],[3,"TryFromIntError"],[4,"TryMergeErrorKind"],[3,"TryMergeError"],[4,"Precision"],[4,"Ordering"],[15,"bool"],[8,"IntoIterator"],[3,"Formatter"],[6,"Result"],[3,"RandomState"],[15,"u8"],[15,"u64"],[15,"usize"],[4,"Option"],[3,"String"],[4,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};