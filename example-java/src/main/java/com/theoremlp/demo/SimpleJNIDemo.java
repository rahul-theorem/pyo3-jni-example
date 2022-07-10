package com.theoremlp.demo;


import java.util.Arrays;

public class SimpleJNIDemo {
    static {
        System.loadLibrary("pyo3_jni");
    }

    public static native int[] range(int max);

    public static void main(String... args) {
        System.out.println(Arrays.toString(range(10)));
    }
}