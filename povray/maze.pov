/* 

   Alok Menghrajani
   Persistence Of Vision raytracer.

Prim's algorithm:  This requires storage  proportional to the  size of
the Maze. During creation, each cell  is one of three types: (1) "In":
The cell  is part of  the Maze and  has been carved into  already, (2)
"Frontier": The cell  is not part of the Maze and  has not been carved
into yet,  but is next to a  cell that's already "in",  and (3) "Out":
The cell is  not part of the  Maze yet, and none of  its neighbors are
"in" either. Start by picking a  cell, making it "in", and setting all
its neighbors to  "frontier". Proceed by picking a  "frontier" cell at
random, and  carving into it from  one of its neighbor  cells that are
"in".  Change that  "frontier" cell  to "in",  and update  any  of its
neighbors that  are "out" to "frontier".  The Maze is  done when there
are no more "frontier" cells left (which means there are no more "out"
cells left  either, so  they're all "in").  This algorithm  results in
Mazes with about as low a  "river" factor as possible, with many short
dead ends, and the solution is  usually pretty direct as well. It also
runs very fast, faster than any  of the others when implemented right. 

*/

global_settings { assumed_gamma 2.2 }

#include "colors.inc"
#include "textures.inc"

camera {
//  up    <0,1,0>
//  right  <1,0,0>
  location <45, 85, -120>/1.3
  look_at <4, 0, -25>
}

light_source { <20, 30, -20> colour White }

background { color rgb <1, 1, 1> }

plane { <0,1,0>, 0
texture{
pigment {image_map {
            jpeg "grass.jpg"
    }
}

 finish {
diffuse 0.3
ambient 0
brilliance 0
}

}
}

#declare mysize = 17;
#declare s1 = 6;
#declare s2 = 5;
#declare sed = seed(23);
#declare reg = 0;
#declare nbFront = 0;

#declare Maze = array[mysize][mysize]
#declare Front = array[2][pow(div(mysize,2),2)]
#declare Copy = array[mysize][mysize]

#declare Count1 = 0;
#while (Count1<mysize)
        #declare Count2 = 0;
        #while (Count2<mysize)
		#declare Copy[Count1][Count2] = 4;				
		#if ((Count1=0) | (Count2=0) | (Count1=(mysize-1)) | (Count2=(mysize-1)) |
			          ((div(Count1,2)=(Count1/2)) | (div(Count2,2)=(Count2/2))))
                	#declare Maze[Count1][Count2]=1;
                #else
                
             	  	#declare Maze[Count1][Count2]=0;
			#declare Copy[Count1][Count2] = 3; 
			
                #end
                #declare Count2=Count2+1;
        #end                
                
        #declare Count1=Count1+1;
#end

//--------------------------------------------------------------//

#macro Make_Front (a,b)
	
	#if (a>2)
		#if (Copy[a-2][b]=3)
			#declare Copy[a-2][b]=2;
			#declare Front[0][nbFront]=a-2;
			#declare Front[1][nbFront]=b;
			#declare nbFront=nbFront+1;
		#end
	#end
  
	#if (a<mysize-2)
		#if (Copy[a+2][b]=3)
  			#declare Copy[a+2][b]=2;
  			#declare Front[0][nbFront]=a+2;
			#declare Front[1][nbFront]=b;
			#declare nbFront=nbFront+1;
		#end
	#end

	#if (b>2)
		#if (Copy[a][b-2]=3)
			#declare Copy[a][b-2]=2;
			#declare Front[0][nbFront]=a;
			#declare Front[1][nbFront]=b-2;
			#declare nbFront=nbFront+1;
		#end
	#end
	
	#if (b<mysize-2)
		#if (Copy[a][b+2]=3)
			#declare Copy[a][b+2]=2;
			#declare Front[0][nbFront]=a;
			#declare Front[1][nbFront]=b+2;
			#declare nbFront=nbFront+1;
		#end
	#end     
#end

#macro Make_In (a,b)
	#declare ok=1;
	#while (ok)
		#declare test=rand(sed)*4;

		#if ((test<1) & (test>0) & (a>2))
			#if (Copy[a-2][b]=1)
				#declare Maze[a-1][b]=0;
				#declare ok=0;
			#end 
			
		#end
		
		#if ((test<2) & (test>1) & (a<mysize-2))
			#if (Copy[a+2][b]=1)
				#declare Maze[a+1][b]=0;
				#declare ok=0;
			#end
		#end
		
		#if ((test<3) & (test>2) & (b>2))
			#if (Copy[a][b-2]=1)
				#declare Maze[a][b-1]=0;
				#declare ok=0;
			#end
		#end
		
		#if ((test<4) & (test>3) & (b<mysize-2))
			#if (Copy[a][b+2]=1)
				#declare Maze[a][b+1]=0;
				#declare ok=0;
			#end
		#end
	#end
#end

//--------------------------------------------------------------//

#declare i = floor(rand(sed)*(mysize-3)/2)*2+1;

#declare Maze[i][0]=0;
#declare Copy[i][1]=1;
Make_Front(i,1)

#declare i = floor(rand(sed)*(mysize-3)/2)*2+1;

#declare Maze[i][mysize-1]=0;
/*#declare Copy[i][mysize-2]=1;
Make_Front(i,mysize-2)   */

//--------------------------------------------------------------//
        
#while (nbFront>0)
	#declare i = floor(rand(sed)*(nbFront*2));
	#if (i>=nbFront)
		#declare i=nbFront-1;
	#end
		
	
	#declare a = Front[0][i];
	#declare b = Front[1][i];
	
	#declare Copy[a][b]=1;
	#declare nbFront=nbFront-1;
	#declare Front[0][i]=Front[0][nbFront];
	#declare Front[1][i]=Front[1][nbFront];
	
	Make_In(a,b)
	
	
	Make_Front(a,b)
	
#end
            


//--------------------------------------------------------------//

union {
    
#declare Count1=0;
#while (Count1 < mysize)
        #declare Count2=0;
        #while (Count2 < mysize)
                #if (Maze[Count1][Count2])
                        box{<Count1*s1, 0, Count2*s1> <Count1*s1+s1, s2, Count2*s1+s1>
  //                      pigment {color Green}

texture {
pigment {image_map {
            jpeg "grass.jpg"
    }
}
 finish {
    ambient 0.3
      diffuse 0.8
    }
}
}
/*                #else
                	box{<Count1*s1, 0, Count2*s1> <Count1*s1+s1, s2, Count2*s1+s1>
                        pigment {color Blue}}   */
                #end
                #declare Count2=Count2+1;
        #end
        #declare Count1=Count1+1;
#end         
        translate <-s1*mysize/2, 0, -s1*mysize/2>
        rotate <0, 0, 0>
}



        
    
