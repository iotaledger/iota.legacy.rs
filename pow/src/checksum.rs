pub fn get_checksum_check(size: usize) -> impl Fn(&[BCTrit]) -> u32 {
    move |&state| {
    static int checkChecksum(Pair<long[], long[]> midCurlState, final int length) {
        Pair<BigInteger[], BigInteger[]> checks = transpose(midCurlState, 0, length);
        int out = -1;
        for(int i = 0; i < Long.SIZE; i++) {
            if(checks.low[i].bitCount() == checks.hi[i].bitCount()) {
                out = i;
                break;
            }
        }
        return out;
    }

    private static long[] identity = new long[Long.SIZE];
    static {
        for(int i = 0; i < identity.length; i++) {
            identity[i] = 1<<i;
        }
    }

    /**
     * This performs a vanilla binary transpose, making rows into columns and vice versa.
     * @param midCurlState
     * @param offset
     * @param length
     * @return transposed pair matrix
     */
    static Pair<BigInteger[], BigInteger[]> transpose(Pair<long[], long[]> midCurlState, final int offset, final int length) {
        Pair<BigInteger[], BigInteger[]> output = new Pair<>(new BigInteger[Long.SIZE], new BigInteger[Long.SIZE]);
        for(int j = 0; j < Long.SIZE; j++) {
            output.low[j] = new BigInteger(new byte[length]);
            output.hi[j] = new BigInteger(new byte[length]);
            for(int i = 0; i < length; i++) {
                if((midCurlState.low[offset + i] & identity[j]) != 0) {
                    output.low[j] = output.low[j].setBit(i);
                }
                if((midCurlState.hi[offset + i] & identity[j]) != 0) {
                    output.hi[j] = output.hi[j].setBit(i);
                }
            }
        }
        return output;
    }
    }
}


