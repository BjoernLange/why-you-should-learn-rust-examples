package com.itemis.borrow.checker.ownership;

import java.util.HashMap;

public final class Main {
    static final class A {
        private String value;

        public A(String value) {
            this.value = value;
        }

        public String getValue() {
            return value;
        }

        public void setValue(String value) {
            this.value = value;
        }

        @Override
        public boolean equals(Object other) {
            if (this == other) {
                return true;
            }
            
            if (other instanceof A) {
                var otherA = (A)other;
                return otherA.value.equals(value);
            } else {
                return false;
            }
        }

        @Override
        public int hashCode() {
            return value.hashCode();
        }
    }

    public static void main(String[] args) {
        var map = new HashMap<A, String>();
        
        var a = new A("A");
        map.put(a, "A");
        map.put(new A("B"), "B");
        map.put(new A("C"), "C");

        a.setValue("D");

        System.out.println("A: " + map.get(new A("A")));
        System.out.println("B: " + map.get(new A("B")));
        System.out.println("C: " + map.get(new A("C")));
        System.out.println("D: " + map.get(new A("D")));
    }
}