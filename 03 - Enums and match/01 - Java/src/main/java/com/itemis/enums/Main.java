package com.itemis.enums;

import java.util.Arrays;

public final class Main {
    static enum A {
        U,
        V,
        W,
        X,
        Y,
        Z,
    }

    private static void doSomething(A a) {
        switch (a) {
            case U:
                System.out.println(a + ": U");
                break;
            case V:
                System.out.println(a + ": V");
                break;
            case X:
                System.out.println(a + ": X");
                break;
            case Y:
                System.out.println(a + ": Y");
                break;
            case Z:
                System.out.println(a + ": Z");
                break;
        }
    }

    public static void main(String[] args) {
        Arrays.stream(A.values()).forEach(Main::doSomething);
    }
}