package com.theoremlp.pex4j;


public class FFIDemo {
    static {
        System.loadLibrary("pex4j_rs");
    }

    public static native String sayHello(String who);

    public static void main(String... args) {
        System.out.println(sayHello("Rahul"));
    }
}