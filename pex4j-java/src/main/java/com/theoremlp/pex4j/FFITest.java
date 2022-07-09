package com.theoremlp.pex4j;


public class FFITest {
    static {
        System.loadLibrary("pex4j-rs");
    }

    public static native String sayHello(String who);

    public static void main(String... args) {
        System.out.println(sayHello("Rahul"));
    }
}