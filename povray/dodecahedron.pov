// Persistence of Vision Ray Tracer Scene Description File
// File: Dodecahedron.pov
// Vers: 4.0
// Desc: Dodecahedron
// Date: May 2000
// Auth: Alok Menghrajani

#version 3.5;
background {color rgb <1, 1, 1>}

// SET UP A BASIC CAMERA AND LIGHT SOURCE

camera {
	location  <0, 1, -20>
	look_at   <0, 0, 0>
}

light_source {
	<20, 18, -15>
	color red 1.0  green 1.0  blue 1.0
}

// DECLARE OUR CONSTANTS AND ARRAYS

#declare radius1 = 0.1;
#declare radius2 = 0.05;
#declare VA = array[20]

// CREATE BASE PENTAGON

#declare i = 0;
#while (i < 5)
	#declare VA[i]=vrotate(<1, 0, 0>, <0, i * 360 / 5, 0>);
#declare i = i + 1;
#end

// CREATE FIVE LOWER LATERAL SIDES

#declare L = vlength(VA[0]-VA[1]);
#declare i = 0;
#while (i < 5)
	#declare VA[i+5]=VA[i]*L;
#declare i = i + 1;
#end

// ROTATE FIVE LOWER LATERAL SIDES
             
#declare gamma = (pi - 2*pi/5)/2;
#declare apx = pi-gamma*2;
#declare PA = cos(apx)*L;
#declare cpx = pi - gamma - 2*gamma;
#declare apc = apx - cpx;
#declare PC = PA / cos(apc);
#declare beta = acos(PC / L );

#declare i = 0;
#while (i < 5)
	#declare VA[i+5]=VA[i]+vaxis_rotate( VA[i+5], vcross(VA[i+5],<0, 1, 0>), beta*180/pi);
	
#declare i = i + 1;
#end             

// CREATE FIVE HIGHER LATERAL SIDES

#declare d = sin(gamma)+1;
#declare i = 0;
#while (i < 5)
	#declare VA[i+10]=-VA[i]*d;
#declare i = i + 1;
#end   

       
// ROTATE FIVE HIGHER LATERAL SIDES       

#declare beta2 = acos((cos(2*gamma-pi/2+cpx)*PC)/(cos(2*gamma-pi/2)*L));

#declare i = 0;
#while (i < 5)
	#declare VA[i+10]=vaxis_rotate( VA[i+10], vcross(VA[i+10],<0, 1, 0>), beta2*180/pi);
	#declare VA[i+10]=VA[i+10]-VA[i]*sin(gamma);
#declare i = i + 1;
#end

// CREATE CAP

#declare G = (VA[5].y + VA[10].y) / 2;

#declare i = 0;
#while (i < 5)
	#declare VA[i+15]=vrotate(<1, 0, 0>, <0, i * 360 / 5 + 180, 0>)+<0, 2*G, 0>;
	#declare i = i + 1;
#end

// DODECAHEDRON READY * RENDER IT

union {
	
	#declare i = 0;
	#while (i < 20)
		sphere {VA[i], radius1}
	#declare i = i + 1;
	#end                     
	
	#declare i=0;
	#while (i < 5)
		cylinder {VA[i], VA[mod(i+1,5)], radius2} 
		cylinder {VA[i], VA[i+5], radius2}
		cylinder {VA[i+10], VA[mod(i+2,5)+5], radius2} 
		cylinder {VA[i+10], VA[mod(i+3,5)+5], radius2}          
		cylinder {VA[i+15], VA[mod(i,5)+10], radius2} 
		cylinder {VA[i+15], VA[mod(i+1,5)+15], radius2}
	#declare i = i + 1;
	#end
	
	texture {
		pigment {color rgb <24/255, 70/255, 164/255>*1.2}
		finish {
			ambient 0.30
			brilliance 2
			diffuse 0.9
			metallic
			specular 0.70
			roughness 1/60
			reflection 0.25
		}
	}

	translate <1, -G, 0>
	scale <4, 4, 4>
}

plane { <0,1,0>, -5*G
	texture{
		pigment {color <1.0, 1.0, 1.0>}
	}

	finish {
		diffuse 1
		ambient 0
		brilliance 0
	}
}
       
// End.
