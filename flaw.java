public class Main
{
	public static void main(String[] args) {
		Point pt = new Point();
	  // pt.setInt(12);
	   //pt.setDouble(1.0);
	   //pt.setBool(true);
	   //pt.setChar('a');
	   //pt.setIntObject(45);
	   //pt.setStringObject("Hello");
		//Even though it's not memory error since the values are initialiazed to their default values
		//this is shouldn't be allowed without initializing it//first with setter methods.
		System.out.println(pt.getInt());
		 System.out.println(pt.getDouble());
		System.out.println(pt.getBool());
		System.out.println(pt.getChar());
		System.out.println(pt.getStringObject());
		System.out.println(pt.getIntObject());
		//This line throws an exception since the default values for object type in java is null.
		System.out.println(pt.charat());
	}
}
class Point {
    //primitive types
    private int a;
    private double b;
    private boolean c;
    private char d;
    
    //Object types
    private String e;
    private Integer f;

    void setInt(int a) {
        this.a = a;
    }

    int getInt() {
        return a;
    }

    void setDouble(double b) {
        this.b = b;
    }
    
    double getDouble() {
        return b;
    }

    void setBool(boolean c) {
        this.c = c;
    }

    boolean getBool() {
        return c;
    }
   
    void setChar(char d){
        this.d=d;
    }
    char getChar(){
        return d;
    }
    void setStringObject(String e){
        this.e=e;
    }
    String getStringObject(){
        return e;
    }
    void setIntObject(Integer f){
        this.f=f;
    }
    Integer getIntObject(){
        return f;
    }
    char charat(){
        return e.charAt(1);
    }
}
