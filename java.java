/******************************************************************************

                            Online Java Compiler.
                Code, Compile, Run and Debug java program online.
Write your code in this editor and press "Run" button to execute it.

*******************************************************************************/
public class Main
{
	public static void main(String[] args) {
		Point pt = new Point();
		//pt.setX(1.1);
		//pt.setY(2.2);
		//pt.setZ(3.3);
		//Even though it's not memory error since the values are initialiazed to zero
		//this is shouldn't be allowed without initializing it first with setter methods.
	    System.out.println(pt.getX());
	    System.out.println(pt.getY());
	    System.out.println(pt.getZ());
	}
}
class Point {
    private double x;
    private double y;
    private double z;

    double getX() {
        return x;
    }

    void setX(double x) {
        this.x = x;
    }

    double getY() {
        return y;
    }

    void setY(double y) {
        this.y = y;
    }

    double getZ() {
        return z;
    }

    void setZ(double z) {
        this.z = z;
    }
}
