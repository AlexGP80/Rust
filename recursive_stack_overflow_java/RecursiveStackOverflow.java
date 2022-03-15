class RecursiveStackOverflow {

  public static void fuckit(int n) {
    System.out.println(n);
    n++;
    RecursiveStackOverflow.fuckit(n);
  }

  public static void main (String args[]) {
    RecursiveStackOverflow.fuckit(1);
  }

}
