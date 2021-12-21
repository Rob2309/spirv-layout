; SPIR-V
; Version: 1.5
; Generator: Google Shaderc over Glslang; 10
; Bound: 140
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %vert "vert" %u_Transform_0 %u_Cam_0 %vIn_position %vIn_color %vIn_normal %vIn_uv %_entryPointOutput_position %_entryPointOutput_worldNormal %_entryPointOutput_uv
               OpSource HLSL 500
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %vert "vert"
               OpName %u_Transform "u_Transform"
               OpMemberName %u_Transform 0 "modelMatrix"
               OpMemberName %u_Transform 1 "invModelMatrix"
               OpName %u_Transform_0 "u_Transform"
               OpName %u_Cam "u_Cam"
               OpMemberName %u_Cam 0 "viewMatrix"
               OpMemberName %u_Cam 1 "projMatrix"
               OpMemberName %u_Cam 2 "invViewMatrix"
               OpMemberName %u_Cam 3 "invProjMatrix"
               OpMemberName %u_Cam 4 "position"
               OpName %u_Cam_0 "u_Cam"
               OpName %vIn_position "vIn.position"
               OpName %vIn_color "vIn.color"
               OpName %vIn_normal "vIn.normal"
               OpName %vIn_uv "vIn.uv"
               OpName %_entryPointOutput_position "@entryPointOutput.position"
               OpName %_entryPointOutput_worldNormal "@entryPointOutput.worldNormal"
               OpName %_entryPointOutput_uv "@entryPointOutput.uv"
               OpMemberDecorate %u_Transform 0 RowMajor
               OpMemberDecorate %u_Transform 0 Offset 0
               OpMemberDecorate %u_Transform 0 MatrixStride 16
               OpMemberDecorate %u_Transform 1 RowMajor
               OpMemberDecorate %u_Transform 1 Offset 64
               OpMemberDecorate %u_Transform 1 MatrixStride 16
               OpDecorate %u_Transform Block
               OpMemberDecorate %u_Cam 0 RowMajor
               OpMemberDecorate %u_Cam 0 Offset 0
               OpMemberDecorate %u_Cam 0 MatrixStride 16
               OpMemberDecorate %u_Cam 1 RowMajor
               OpMemberDecorate %u_Cam 1 Offset 64
               OpMemberDecorate %u_Cam 1 MatrixStride 16
               OpMemberDecorate %u_Cam 2 RowMajor
               OpMemberDecorate %u_Cam 2 Offset 128
               OpMemberDecorate %u_Cam 2 MatrixStride 16
               OpMemberDecorate %u_Cam 3 RowMajor
               OpMemberDecorate %u_Cam 3 Offset 192
               OpMemberDecorate %u_Cam 3 MatrixStride 16
               OpMemberDecorate %u_Cam 4 Offset 256
               OpDecorate %u_Cam Block
               OpDecorate %u_Cam_0 DescriptorSet 0
               OpDecorate %u_Cam_0 Binding 0
               OpDecorate %vIn_position Location 0
               OpDecorate %vIn_color Location 1
               OpDecorate %vIn_normal Location 2
               OpDecorate %vIn_uv Location 3
               OpDecorate %_entryPointOutput_position BuiltIn Position
               OpDecorate %_entryPointOutput_worldNormal Location 0
               OpDecorate %_entryPointOutput_uv Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
    %v2float = OpTypeVector %float 2
    %v4float = OpTypeVector %float 4
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
    %float_1 = OpConstant %float 1
%mat4v4float = OpTypeMatrix %v4float 4
%u_Transform = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_PushConstant_u_Transform = OpTypePointer PushConstant %u_Transform
%u_Transform_0 = OpVariable %_ptr_PushConstant_u_Transform PushConstant
%_ptr_PushConstant_mat4v4float = OpTypePointer PushConstant %mat4v4float
      %u_Cam = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %mat4v4float %v3float
%_ptr_Uniform_u_Cam = OpTypePointer Uniform %u_Cam
    %u_Cam_0 = OpVariable %_ptr_Uniform_u_Cam Uniform
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
      %int_1 = OpConstant %int 1
    %float_0 = OpConstant %float 0
%_ptr_Input_v3float = OpTypePointer Input %v3float
%vIn_position = OpVariable %_ptr_Input_v3float Input
  %vIn_color = OpVariable %_ptr_Input_v3float Input
 %vIn_normal = OpVariable %_ptr_Input_v3float Input
%_ptr_Input_v2float = OpTypePointer Input %v2float
     %vIn_uv = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_entryPointOutput_position = OpVariable %_ptr_Output_v4float Output
%_ptr_Output_v3float = OpTypePointer Output %v3float
%_entryPointOutput_worldNormal = OpVariable %_ptr_Output_v3float Output
%_ptr_Output_v2float = OpTypePointer Output %v2float
%_entryPointOutput_uv = OpVariable %_ptr_Output_v2float Output
       %vert = OpFunction %void None %3
          %5 = OpLabel
         %75 = OpLoad %v3float %vIn_position
         %81 = OpLoad %v3float %vIn_normal
         %85 = OpLoad %v2float %vIn_uv
        %115 = OpCompositeExtract %float %75 0
        %116 = OpCompositeExtract %float %75 1
        %117 = OpCompositeExtract %float %75 2
        %118 = OpCompositeConstruct %v4float %115 %116 %117 %float_1
        %119 = OpAccessChain %_ptr_PushConstant_mat4v4float %u_Transform_0 %int_0
        %120 = OpLoad %mat4v4float %119
        %121 = OpVectorTimesMatrix %v4float %118 %120
        %122 = OpAccessChain %_ptr_Uniform_mat4v4float %u_Cam_0 %int_0
        %123 = OpLoad %mat4v4float %122
        %124 = OpVectorTimesMatrix %v4float %121 %123
        %125 = OpAccessChain %_ptr_Uniform_mat4v4float %u_Cam_0 %int_1
        %126 = OpLoad %mat4v4float %125
        %127 = OpVectorTimesMatrix %v4float %124 %126
        %131 = OpCompositeExtract %float %81 0
        %132 = OpCompositeExtract %float %81 1
        %133 = OpCompositeExtract %float %81 2
        %134 = OpCompositeConstruct %v4float %131 %132 %133 %float_0
        %135 = OpAccessChain %_ptr_PushConstant_mat4v4float %u_Transform_0 %int_1
        %136 = OpLoad %mat4v4float %135
        %137 = OpTranspose %mat4v4float %136
        %138 = OpVectorTimesMatrix %v4float %134 %137
        %139 = OpVectorShuffle %v3float %138 %138 0 1 2
               OpStore %_entryPointOutput_position %127
               OpStore %_entryPointOutput_worldNormal %139
               OpStore %_entryPointOutput_uv %85
               OpReturn
               OpFunctionEnd
