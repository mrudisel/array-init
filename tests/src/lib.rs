use array_init::array;


#[test]
pub fn test() {
    let arr: [Vec<u8>; 5] = array![Vec::new; 5];

    assert_eq!(arr.len(), 5)
}
