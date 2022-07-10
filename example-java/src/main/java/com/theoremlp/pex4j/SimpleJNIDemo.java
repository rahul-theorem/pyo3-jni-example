package com.theoremlp.pex4j;


import java.util.Arrays;

public class SimpleJNIDemo {
    static {
        System.loadLibrary("pex4j_rs");
    }

    public static native int[] genRange();

    public static void main(String... args) {
        System.out.println(Arrays.toString(genRange()));
    }
}