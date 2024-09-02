This is an ongoing development project for a GPU based ray-tracer,
initially based on Ray Tracing in One Weekend.

The backbone of this project is what I used to call WiW 
for Wgpu-Imgui-Winit (the frameworks I used to implement this), 
though the imgui crates needed are on my local machine as imgui hasn't upgraded
to the latest Winit version as yet.  

I'm currently only rendering the final scene is RTiOW, though I used a BVH implemented
with the SAH. I'm also trying to clock the megakernel (compute only)
using write_timestamps, though there are two issues: first, the timestamps occasionally
come back as zeros; second, the wait for result seems to mess up the winit loop when
I try to resize.

Progress:
- updated camera and created camera_controller
- created common_code directory for better overall code management b/w gpu and cpu tracers
- updated cpu version of tracer for debugging purposes and added it to workgroup
- added imgui; local imgui files that I modified at this point
- runs samples/frame until total samples/pixel is complete; shows progress in gui
- changed ray tracing generation algorithm from RTiOW to projection and view matrix based
- final pixel color has a sqrt taken; need to investigate colors more

To do (in no particular order):
- get the cpu code running using multiple threads
- figure out the bug in the screen resize where the screen goes black
- see if I can optimize this render time; I feel like 300ms for this image is too long, and I want to do real-time rendering
- figure out how to accurately get framerate and compute times
- implement a wavefront path tracing algorithm
- add other shapes (triangles, planes, quads, planes, etc)
- add an interactive camera
- add more complex rendering ideas from PBR book
- load in more complex obj files for some cool pictures
