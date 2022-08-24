# 属性动画库

## 结构设计

* 设计需求:
  * 需要为 目标数据对象 的 属性 进行 动画运算
  * 目标数据对象 的 类型是不确定的
  * 目标数据对象 的 属性位置 是不确定的
  * 属性 的数据类型是不确定的
* 动画数据运算需要的过程
  * 计算出动画的运行进度
  * 计算出运行进度对应的动画曲线上的值
  * 将动画曲线值应用到 动画数据
  * 运行进度、动画曲线 在此设计使用 f32 类型
* 提取抽象
  * 用无符号数字ID 映射 目标数据对象
  * 用无符号数字ID 映射 目标属性位置
    * 一个目标数据对象需要动画的属性数量 256 应该足够, 因此使用 u8
    * 目标属性位置 完全只能由 目标数据对象 决定, 因此可以在目标数据对象实现的地方 实现动画属性枚举, 枚举在使用时可转换为 u8
  * 属性数据类型 需要做限定, 因为该数据类型需要动画运算
    * 动画数据类型 需要支持
      * 与自身类型 的 Add
      * 与动画曲线数值类型 (f32) 的 相乘
  * 动画数据关键帧与 动画进度计算分离,再用 用无符号数字ID 进行关联
    * 用无符号数字ID 映射 属性数据类型
* 类型层次
  * 目标数据对象ID 分配器
    * IDAnimatableTargetAllocator
  * 属性数据类型ID 分配器
    * KeyFrameDataTypeAllocator
    * 一种类型的数据一次性永久分配获得ID
  * 动画进度曲线计算器
    * 在此封装数据类型无关的动画控制结构
      * AnimationContextAmount - 动画进度计算 & 动画事件处理
        * AnimationGroup - 可对多个动画打包为动画组
          * TargetAnimation - 关联了目标对象和一个属性动画的中间数据结构
            * Animation - 一个属性动画
  * 对应数据类型的动画关键帧缓存管理和计算上下文
    * TypeAnimationContext<T>