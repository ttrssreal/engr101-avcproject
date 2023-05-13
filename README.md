# AVCProject Team 28

**EMAILS**

Samuel Catalan: catalasamu@myvuw.ac.nz

James Mark Adair: churchjame2@myvuw.ac.nz

Viraj Patel: patelvira2@myvuw.ac.nz

**ROLES**

Coordinator: Viraj Patel, manages the schedule of the project, with the various dates and meetings.

Tester: Samuel Catalan, the tester designs, creates and tests the code that the robot will use. 

Architect: James Mark Adair, they are in charge of the design of the robot being used and buidling it. 




**GENERAL IDEA OF SOFTWARE** 

The general of idea of the software is that it will be comprised of multiple programs.
Firstly it will begin with initialising all the hardware that the robot will be using to move, like connecting the raspberry pi, to the wifi and using that connecting to the move the robot. 

It will then begin decide which quadrant that it is currently in, as each quadrant has diffrerent obstacles. Then depending on the quadrant the software will then go to an infinite loops, where the software contacts the robot API, and the specific quadrant sensor algorithm is used, and then a corrospoinding, movement program for the quadrant is used, to move the robot, adjusting it’s motors. 



**ROAD MAP**
| Start date | Task Decription | Responsible Team Member | End Date | Outcome |
|----------|----------|----------|----------|----------|
|9/05/2023|Develop the project plan|Viraj Patel|13/05/2023||
|16/05/2023|Design robot frame/components|James Mark Adair|17/05/2023|          |
|16/05/2023|Construct the robot based on the design|James Mark Adair|17/05/2023|          |
|17/05/2023|Connect and establish a connection to the robot via wifi, and check if it can exchange data|Samuel Catalan|23/05/2023|          | 
|17/05/2023|Begin developing a program to control the robot with basic movements (based on commands)|Samuel Catalan|25/05/2023|          | 
|17/05/2023|Test the robot to check whether it can execute these movements from the server|Samuel Catalan, Viraj Patel|25/05/2023|          | 
|24/05/2023|Code error identifying mechanisms/possible recovery procedure (data loss)|Samuel Catalan|25/05/2023|          | 
|23//05/2023|Develop quadrant 1: Open the gate by exchanging with the server over Wifi|Samuel Catalan|25/05/2023|          | 
|23/05/2023|Verify and run multiple tests under different conditions (with errors) and document the results|Samuel Catalan, Viraj Patel|25/05/2023|          | 
|23/05/2023|Refine the code/design for quadrant 1 (based on the success of the test)|Samuel Catalan, Viraj Patel|25/05/2023|          | 
|23/05/2023|Design idea for quadrant 2 (sensors), and make physical changes (if necessary)|Samuel Catalan, James Mark Adair|25/05/2023|          | 
|24/05/2023|Develop a sensor program/algorithm to detect/follow the squiggly line and control movement|Samuel Catalan|25/05/2023|          | 
|24/05/2023|Verify and test quadrant 2 under different conditions, and document the results|Samuel Catalan, Viraj Patel|25/05/2023|          | 
|24/05/2023|Refine the code/design for quadrant 2 (based on the success of the test)|Samuel Catalan, Viraj Patel|25/05/2023|          | 
|30/05/2023|Design idea for line detection (possible algorithm to get through sharp turns)|Samuel Catalan, James Mark Adair|1/06/2023|          | 
|30/05/2023|Develop a program for quadrant 3 (logic of direction selection)|Samuel Catalan|1/06/2023|          | 
|30/05/2023|Verify and test quadrant 3 under different conditions, and document the results|Samuel Catalan, Viraj Patel|1/06/2023|          | 
|30/05/2023|Refine code/design for quadrant 3 (based on the success of the test)|Samuel Catalan, Viraj Patel|1/06/2023|          | 
|31/05/2023|Design an idea for detecting the coloured cylinders, and avoiding them (another algorithm)|Samuel Catalan, James Mark Adair|1/06/2023|          | 
|31/05/2023|Implement idea, and develop a program for quadrant 4|Samuel Catalan|31/05/2023|          | 
|31/05/2023|Verify and test quadrant 4 under different conditions, and document the results|Samuel Catalan, Viraj Patel|1/05/2023|          | 
|31/05/2023|Refine code/design for quadrant 4 (based off the success of the tests)|Samuel Catalan, Viraj Patel|1/05/2023|          | 
|Week 12 (29 May - 2nd June)|Present completed project to stakeholders|Viraj Patel|3/05/2023|          | 
|Week 12|AVC Final report|Samuel Catalan, Viraj Patel, James Mark Adair|3rd June|          | 

