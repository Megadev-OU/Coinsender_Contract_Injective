; ModuleID = 'probe7.be352f288e443b2d-cgu.0'
source_filename = "probe7.be352f288e443b2d-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h7380df6f6195177aE"(ptr sret([8 x i8]) align 1 %_0, double %self) unnamed_addr #0 {
start:
  %self1 = bitcast double %self to i64
  store i64 %self1, ptr %_0, align 1
  ret void
}

; probe7::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe75probe17hf83ffb24e5c6038fE() unnamed_addr #1 {
start:
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  call void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h7380df6f6195177aE"(ptr sret([8 x i8]) align 1 %_1, double 3.140000e+00) #2
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.73.0-nightly (0bdb00d55 2023-08-15)"}
