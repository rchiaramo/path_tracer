This is an ongoing development project for a GPU based ray-tracer,
initially based on Ray Tracing in One Weekend.

The backbone of this project is what I used to call WiW 
for Wgpu-Imgui-Winit (the frameworks I used to implement this), 
though the imgui crates needed are on my local machine as imgui hasn't upgraded
to the latest Winit version as yet.  

I'm currently only rendering the final scene is RTiOW, though I used a BVH implemented
with the SAH. I'm also trying to clock the megakernel (compute only)
using write_timestamps, though the timestamps occasionally come back as zeros.

At this point in my progress, the next step is to implement a wavefront pathtracer.  I will
copy this code to a new repository, path_tracer_wavefront, and start fresh there with this
as the starting point.  I want to keep this in a working state for future comparisons.

Progress:
- basic camera movement implemented
- cpu code now runs with threads using rayon
- fixed resize screen bug
- updated camera and created camera_controller
- created common_code directory for better overall code management b/w gpu and cpu tracers
- updated cpu version of tracer for debugging purposes and added it to workgroup
- added imgui; local imgui files that I modified at this point
- runs samples/frame until total samples/pixel is complete; shows progress in gui
- changed ray tracing generation algorithm from RTiOW to projection and view matrix based
- final pixel color has a sqrt taken; need to investigate colors more

To do (in no particular order):
- see if I can optimize this render time; I feel like 300ms for this image is too long, and I want to do real-time rendering
- implement a wavefront path tracing algorithm
- add other shapes (triangles, planes, quads, planes, etc)
- add more complex rendering ideas from PBR book
- load in more complex obj files for some cool pictures
