struct SecretKeyKesProduct;
struct VerificationKesProductKey;

struct KesBinaryTree;

struct SumComposition;

type SIG = ([u8], [u8], Vec<[u8]>);
type VK = ([u8], i32);
type SK = KesBinaryTree;

struct ProductComposition {
    sum_composition: SumComposition,
}

impl ProductComposition {}

struct KesProduct;

impl KesProduct {
    pub fn create_key_pair(
        _seed: u8,
        _height: (i16, i16),
        _offset: i32,
    ) -> (SecretKeyKesProduct, VerificationKesProductKey) {
        return (SecretKeyKesProduct, VerificationKesProductKey);
    }
}
