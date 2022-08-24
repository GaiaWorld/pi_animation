
#[derive(Debug)]
pub enum EAnimationError {
    None = 0,
    NotFoundAttr,
    NotNeedModify,
    KeyTargetCannotAllocMore,
    KeyTargetAttrCannotAllocMore,
    KeyAnimeDataTypeCannotAllocMore,
    AnimationFramePerSecondCannotZero,
    FrameCurveNotFound,
    AnimationGroupNotFound,
    AnimationGroupHasStarted,
    AnimationGroupNotPlaying,
    RuntimeInfoMapNotFindType,
    TargetIDNotRecordForTypeAnimationContext,
}