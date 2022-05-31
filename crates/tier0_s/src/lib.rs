#![allow(unused_variables)]
#![feature(pointer_byte_offsets)]

use esteem_util::stub;

stub!(AllocateCrashMemoryReserve);
stub!(AreStackTrackingFiltersEnabledAtStart);
stub!(AssertMsgImplementation);
stub!(AssertMsgImplementationF);
stub!(AssertMsgImplementationV);
stub!(BBlockingGetMiniDumpLock);
stub!(BGetLocalFQDN);
stub!(BGetMiniDumpLock);
stub!(BWritingFatalMiniDump);
stub!(BWritingMiniDump);
stub!(BWritingNonFatalMiniDump);
stub!(CVProfile_ExitScope);
stub!(CallAssertFailedNotifyFunc);
stub!(CallFlushLogFunc);
stub!(CatchAndWriteMiniDump);
stub!(CatchAndWriteMiniDumpEx);
stub!(CatchAndWriteMiniDumpExForVoidPtrFn);
stub!(CatchAndWriteMiniDumpExReturnsInt);
stub!(CatchAndWriteMiniDumpForVoidPtrFn);
stub!(ClearStackTrackingFilters);
stub!(ClearWritingMiniDump);
stub!(CrackSmokingCompiler);
stub!(CreateSimpleProcess);
stub!(DLog);
stub!(DWarning);
stub!(DeclareCurrentThreadIsMainThread);
stub!(DoNewAssertDialog);
stub!(ETWBegin);
stub!(ETWEnd);
stub!(ETWIsTracingEnabled);
stub!(ETWMark);
stub!(ETWMark1I);
stub!(ETWMark1S);
stub!(ETWMark2I);
stub!(ETWMark2S);
stub!(ETWMark3I);
stub!(ETWMarkPrintf);
stub!(ETWOverlayFrameMark);
stub!(ETWRenderFrameMark);
stub!(ETW_Steamworks_DispatchCallback_End_);
stub!(ETW_Steamworks_DispatchCallback_Start);
stub!(ETW_Steamworks_IPCRecv_End_);
stub!(ETW_Steamworks_IPCRecv_Start);
stub!(ETW_Steamworks_IPCSend_End_);
stub!(ETW_Steamworks_IPCSend_Start);
stub!(ETW_Steamworks_RunCallbacks_End_);
stub!(ETW_Steamworks_RunCallbacks_Start);
stub!(ETW_Streaming_SendPacket);
stub!(EnableCrashingOnCrashes);
stub!(Error);
stub!(FreeCrashMemoryReserve);
stub!(GetAvailableRAM);
stub!(GetCRunTimeVersion);
stub!(GetCrashHandlerFactory);
stub!(GetInstalledRAM);
stub!(GetLocalHostname);
stub!(GetMiniDumpBuildID);
stub!(GetMiniDumpDirectory);
stub!(GetMiniDumpSteamID);
stub!(GetNumberOfSpewAndLogGroups);
stub!(GetNumberOfStackTrackingFilters);
stub!(GetPortableSystemInformation);
stub!(GetSpewAndLogLevel);
stub!(GetSpewAndLogLevelByGroupIndex);
stub!(GetSpewOutputFunc);
stub!(GetStackTrackingFilter);
stub!(HasStackTrackingFilters);
stub!(InitPME);
stub!(InitializeStackTrackingFilters);
stub!(Is64BitOS);
stub!(IsInAssert);
stub!(IsInFatalAssert);
stub!(IsLogActive);
stub!(IsSpewActive);
stub!(IsStackTrackingFiltered);
stub!(Log);
stub!(MiniDumpUnlock);
stub!(Msg);
stub!(Plat_AbsoluteTime);
stub!(Plat_AbsoluteTimeToFloat);
stub!(Plat_Alloc);
stub!(Plat_AttachDebuggerToProcess);
stub!(Plat_CommandLineParamExists);
stub!(Plat_CommandLineParamValue);
stub!(Plat_DefaultAllocErrorFn);
stub!(Plat_FloatTime);
stub!(Plat_Free);
stub!(Plat_GetExecutablePath);
stub!(Plat_GetExecutablePathUTF8);
stub!(Plat_GetPackageName);
stub!(Plat_GetProcessArgv);
stub!(Plat_GetStackBackTrace);
stub!(Plat_InternalOverrideArgv);
stub!(Plat_IsChromeOS);
stub!(Plat_IsGamepadUI);
stub!(Plat_IsGamescope);
stub!(Plat_IsInDebugSession);
stub!(Plat_IsSteamConsoleMode);
stub!(Plat_IsSteamDeck);
stub!(Plat_IsSteamOS);
stub!(Plat_IsSteamOS3);
stub!(Plat_MSTime);
stub!(Plat_MSTime64);
stub!(Plat_OutputDebugString);
stub!(Plat_OutputDebugStringRaw);
stub!(Plat_Realloc);
stub!(Plat_RelativeTickFrequency);
stub!(Plat_RelativeTicks);
stub!(Plat_SetAllocErrorFn);
stub!(Plat_TickAddMicroSec);
stub!(Plat_TickDiffMicroSec);
stub!(Plat_TickDiffMilliSec);
stub!(Plat_USTime);
stub!(Plat_VirtualAccessFlags);
stub!(Plat_VirtualAlloc);
stub!(Plat_VirtualFree);
stub!(Plat_VirtualProtect);
stub!(Plat_asctime);
stub!(Plat_ctime);
stub!(Plat_daylight);
stub!(Plat_gmtime);
stub!(Plat_localtime);
stub!(Plat_timegm);
stub!(Plat_timezone);
stub!(RealGetCallStack);
stub!(RealPrintCallStack);
stub!(RealPrintRawCallStack);
stub!(ReleaseThreadHandle);
stub!(SecureRandomBytes);
stub!(SetAssertDumpStack);
stub!(SetAssertFailedNotifyFunc);
stub!(SetFlushLogFunc);
stub!(SetFullMemoryDumpOnCrash);
stub!(SetInAssert);
stub!(SetInFatalAssert);
stub!(SetMiniDumpAppID);
stub!(SetMiniDumpBuildID);
stub!(SetMiniDumpCustomInfo);
stub!(SetMiniDumpSteamID);
stub!(SetStackTrackingFilter);
stub!(ShouldUseNewAssertDialog);
stub!(ShutdownPME);
stub!(SpewActivate);
stub!(SpewAndLogActivate);
stub!(SpewAndLogChangeIfStillDefault);
stub!(SpewChangeIfStillDefault);
stub!(SpewOutputFunc);
stub!(SpewWrittenMiniDumps);
stub!(SteamRealPath);
stub!(TSListBase_Init);
stub!(TSListBase_Pop);
stub!(TSListBase_Push);
stub!(TSListBase_SwapList);
stub!(TSListBase_UnsafePeek);
stub!(TSQueueBase_Init);
stub!(TSQueueBase_Pop);
stub!(TSQueueBase_Push);
stub!(TSQueueBase_UnsafeDummy);
stub!(TSQueueBase_UnsafePeek);
stub!(TSQueue_PopIntoFreeList);
stub!(TSQueue_UnsafeDebugCheck);
stub!(TestThread_Yield);
stub!(Test_HasFailed);
stub!(Test_HasFinished);
stub!(Test_IsActive);
stub!(Test_RunFrame);
stub!(Test_RunTest);
stub!(Test_SetFailed);
stub!(Test_TerminateThread);
stub!(ThreadCloseProcess);
stub!(ThreadFindProcessByName);
stub!(ThreadGetCurrentHandle);
stub!(ThreadGetCurrentId);
stub!(ThreadGetCurrentProcessHandle);
stub!(ThreadGetCurrentProcessId);
stub!(ThreadGetCurrentRunningRef);
stub!(ThreadGetPriority);
stub!(ThreadGetProcessExitCode);
stub!(ThreadGetProcessId);
stub!(ThreadGetProcessListInfo);
stub!(ThreadImplOneTimeInit);
stub!(ThreadInMainThread);
stub!(ThreadInterlockedAssignIf);
stub!(ThreadInterlockedAssignIf64);
stub!(ThreadInterlockedAssignPointerIf);
stub!(ThreadInterlockedCompareExchange);
stub!(ThreadInterlockedCompareExchange64);
stub!(ThreadInterlockedCompareExchangePointer);
stub!(ThreadInterlockedDecrement);
stub!(ThreadInterlockedDecrement64);
//stub!(ThreadInterlockedExchange);
stub!(ThreadInterlockedExchange64);
stub!(ThreadInterlockedExchangeAdd);
stub!(ThreadInterlockedExchangeAdd64);
stub!(ThreadInterlockedExchangePointer);
stub!(ThreadInterlockedIncrement);
stub!(ThreadInterlockedIncrement64);
stub!(ThreadIsProcessActive);
stub!(ThreadIsProcessIdActive);
stub!(ThreadIsThreadRunning);
stub!(ThreadMicroSleepPOSIX);
stub!(ThreadOpenProcess);
stub!(ThreadResumeProcess);
stub!(ThreadSetAffinity);
stub!(ThreadSetBackgroundPriority);
stub!(ThreadSetDebugName);
stub!(ThreadSetPriority);
stub!(ThreadShellExecute);
stub!(ThreadShellExecuteNoWindow);
stub!(ThreadSleep);
stub!(ThreadSpinWaitForValue);
stub!(ThreadSuspendProcess);
stub!(ThreadTerminate);
stub!(ThreadTerminateProcessCode);
stub!(ThreadWaitForProcessExit);
stub!(VProfInternalEnterScopeCurrentThread);
stub!(VProfInternalGetProfileForCurrentThread);
stub!(ValidateSpew);
stub!(Warning);
stub!(WriteMiniDump);
stub!(_DMsg);
stub!(_SpewMessageType);
stub!(_SpewMessageTypeSourceFmt);
stub!(_SpewMessageTypeSourceV);
stub!(_Z17GetCPUInformationv);
stub!(_ZN10CValidator10FindObjectEPv);
stub!(_ZN10CValidator11ClaimMemoryEPv);
stub!(_ZN10CValidator11ClaimOsFileEi);
stub!(_ZN10CValidator11ClaimSocketEi);
stub!(_ZN10CValidator11DiffAgainstEPS_);
stub!(_ZN10CValidator11RenderLeaksEj);
stub!(_ZN10CValidator13RenderObjectsEj);
stub!(_ZN10CValidator15ClaimConnectionEj);
stub!(_ZN10CValidator15ClaimTrackedKeyEP11CTrackItemsPvPj);
stub!(_ZN10CValidator16ClaimArrayMemoryEPv);
stub!(_ZN10CValidator17AddValidationLockEPN16SteamThreadTools12CThreadMutexE);
stub!(_ZN10CValidator18SetAllocSizeFilterEi);
stub!(_ZN10CValidator19ClaimMemory_AlignedEPv);
stub!(_ZN10CValidator20ClaimUntrackedMemoryEj);
stub!(_ZN10CValidator21UnlockValidationLocksEv);
stub!(_ZN10CValidator30BExcludeAllocationFromTrackingEPKci);
stub!(_ZN10CValidator3PopEv);
stub!(_ZN10CValidator4PushEPKcPvS1_);
stub!(_ZN10CValidator8FinalizeEv);
stub!(_ZN10CValidator8ValidateERS_PKc);
stub!(_ZN10CValidator9IsClaimedEPv);
stub!(_ZN10CValidatorC1EibP20IMemBlockClaimedList);
stub!(_ZN10CValidatorC2EibP20IMemBlockClaimedList);
stub!(_ZN10CValidatorD1Ev);
stub!(_ZN10CValidatorD2Ev);
stub!(_ZN10SteamVProf10CVProfNode10EnterScopeEPv);
stub!(_ZN10SteamVProf10CVProfNode10GetSubNodeEPKc17EVProfBugdetGroup);
stub!(_ZN10SteamVProf10CVProfNode15SetCurFrameTimeEm);
stub!(_ZN10SteamVProf10CVProfNode22GetCurTimeLessChildrenEv);
stub!(_ZN10SteamVProf10CVProfNode23GetPrevTimeLessChildrenEv);
stub!(_ZN10SteamVProf10CVProfNode24GetTotalTimeLessChildrenEv);
stub!(_ZN10SteamVProf10CVProfNode5PauseEv);
stub!(_ZN10SteamVProf10CVProfNode5ResetEv);
stub!(_ZN10SteamVProf10CVProfNode6ResumeEv);
stub!(_ZN10SteamVProf10CVProfNode8ValidateER10CValidatorPKc);
stub!(_ZN10SteamVProf10CVProfNode9ExitScopeEv);
stub!(_ZN10SteamVProf10CVProfNode9MarkFrameEv);
stub!(_ZN10SteamVProf10CVProfNode9ResetPeakEv);
stub!(_ZN10SteamVProf10CVProfNodeC1EPNS_9CVProfileEPKcPS0_17EVProfBugdetGroup);
stub!(_ZN10SteamVProf10CVProfNodeC2EPNS_9CVProfileEPKcPS0_17EVProfBugdetGroup);
stub!(_ZN10SteamVProf10CVProfNodeD1Ev);
stub!(_ZN10SteamVProf10CVProfNodeD2Ev);
stub!(_ZN10SteamVProf13CVProfManager16EnableDumpSpikesEi);
stub!(_ZN10SteamVProf13CVProfManager18BIsProfilePtrValidEPNS_9CVProfileE);
stub!(_ZN10SteamVProf13CVProfManager19AddProfileForThreadEPNS_9CVProfileEjj);
stub!(_ZN10SteamVProf13CVProfManager20GetAllThreadProfilesEPNS_14CVProfileArrayE);
stub!(_ZN10SteamVProf13CVProfManager21CleanupUnusedProfilesEv);
stub!(_ZN10SteamVProf13CVProfManager21DumpAllThreadProfilesEi);
stub!(_ZN10SteamVProf13CVProfManager23StopProfilingAllThreadsEv);
stub!(_ZN10SteamVProf13CVProfManager24StartProfilingAllThreadsEv);
stub!(_ZN10SteamVProf13CVProfManager8ValidateER10CValidatorPKc);
stub!(_ZN10SteamVProf13CVProfManagerC1Ev);
stub!(_ZN10SteamVProf13CVProfManagerC2Ev);
stub!(_ZN10SteamVProf13CVProfManagerD1Ev);
stub!(_ZN10SteamVProf13CVProfManagerD2Ev);
stub!(_ZN10SteamVProf14CVProfileArrayD1Ev);
stub!(_ZN10SteamVProf14CVProfileArrayD2Ev);
stub!(_ZN10SteamVProf20CVProfileThreadEntry11DumpProfileEi);
stub!(_ZN10SteamVProf20CVProfileThreadEntry13DeleteProfileEv);
stub!(_ZN10SteamVProf20CVProfileThreadEntry13StopProfilingEv);
stub!(_ZN10SteamVProf20CVProfileThreadEntry14StartProfilingEv);
stub!(_ZN10SteamVProf20CVProfileThreadEntry16OnFrameCompletedEv);
stub!(_ZN10SteamVProf20CVProfileThreadEntry17OnNewFrameEnteredEv);
stub!(_ZN10SteamVProf20CVProfileThreadEntry22SetDumpSpikesThresholdEi);
stub!(_ZN10SteamVProf20CVProfileThreadEntryC1EPNS_9CVProfileEjj);
stub!(_ZN10SteamVProf20CVProfileThreadEntryC2EPNS_9CVProfileEjj);
stub!(_ZN10SteamVProf9CVProfile10DumpSortedEPKcdPFbRKNS_10TimeSums_tES5_Ei);
stub!(_ZN10SteamVProf9CVProfile10EnterScopeEPKc17EVProfBugdetGroupPv);
stub!(_ZN10SteamVProf9CVProfile10ResetPeaksEv);
stub!(_ZN10SteamVProf9CVProfile11FreeNodes_REPNS_10CVProfNodeE);
stub!(_ZN10SteamVProf9CVProfile11GetThreadIDEv);
stub!(_ZN10SteamVProf9CVProfile12OutputReportEiPKci);
stub!(_ZN10SteamVProf9CVProfile18AddBudgetGroupNameE17EVProfBugdetGroupiPKc);
stub!(_ZN10SteamVProf9CVProfile18CreateBudgetGroupsEv);
stub!(_ZN10SteamVProf9CVProfile19GetBudgetGroupColorE17EVProfBugdetGroupRiS2_S2_S2_);
stub!(_ZN10SteamVProf9CVProfile23RegisterCallbackHandlerEPNS_24IVProfileCallbackHandlerE);
stub!(_ZN10SteamVProf9CVProfile25UnregisterCallbackHandlerEPNS_24IVProfileCallbackHandlerE);
stub!(_ZN10SteamVProf9CVProfile40GetFrameTimeOutsideBudgetGroup_RecursiveEPNS_10CVProfNodeE17EVProfBugdetGroup);
stub!(_ZN10SteamVProf9CVProfile44BProfileHasNodesOutsideBudgetGroup_RecursiveEPNS_10CVProfNodeEi);
stub!(_ZN10SteamVProf9CVProfile4DumpEPKcz);
stub!(_ZN10SteamVProf9CVProfile4StopEv);
stub!(_ZN10SteamVProf9CVProfile4TermEv);
stub!(_ZN10SteamVProf9CVProfile5PauseEv);
stub!(_ZN10SteamVProf9CVProfile5ResetEv);
stub!(_ZN10SteamVProf9CVProfile5StartEv);
stub!(_ZN10SteamVProf9CVProfile6ResumeEv);
stub!(_ZN10SteamVProf9CVProfile8FindNodeEPNS_10CVProfNodeEPKc);
stub!(_ZN10SteamVProf9CVProfile8SumTimesEPKci);
stub!(_ZN10SteamVProf9CVProfile8SumTimesEPNS_10CVProfNodeEi);
stub!(_ZN10SteamVProf9CVProfile8ValidateER10CValidatorPKc);
stub!(_ZN10SteamVProf9CVProfile9DumpNodesEPNS_10CVProfNodeEib);
stub!(_ZN10SteamVProf9CVProfile9ExitScopeEv);
stub!(_ZN10SteamVProf9CVProfile9MarkFrameEPKc);
stub!(_ZN10SteamVProf9CVProfileC1Ev);
stub!(_ZN10SteamVProf9CVProfileC2Ev);
stub!(_ZN10SteamVProf9CVProfileD1Ev);
stub!(_ZN10SteamVProf9CVProfileD2Ev);
stub!(_ZN15AssertMsgHelperILb0EE12AssertFailedEPKcjS2_z);
stub!(_ZN15AssertMsgHelperILb1EE12AssertFailedEPKcjS2_);
stub!(_ZN16SteamThreadTools12CThreadEvent3SetEv);
stub!(_ZN16SteamThreadTools12CThreadEvent5CheckEv);
stub!(_ZN16SteamThreadTools12CThreadEvent5ResetEv);
stub!(_ZN16SteamThreadTools12CThreadEventC1EPKcbb);
stub!(_ZN16SteamThreadTools12CThreadEventC1Eb);
stub!(_ZN16SteamThreadTools12CThreadEventC2EPKcbb);
stub!(_ZN16SteamThreadTools12CThreadEventC2Eb);
//stub!(_ZN16SteamThreadTools12CThreadMutexC1Ev);
stub!(_ZN16SteamThreadTools12CThreadMutexC2Ev);
stub!(_ZN16SteamThreadTools12CThreadMutexD1Ev);
stub!(_ZN16SteamThreadTools12CThreadMutexD2Ev);
stub!(_ZN16SteamThreadTools13CThreadRWLock11UnlockWriteEv);
stub!(_ZN16SteamThreadTools13CThreadRWLock11WaitForReadEv);
stub!(_ZN16SteamThreadTools13CThreadRWLock12LockForWriteEv);
stub!(_ZN16SteamThreadTools16CThreadFullMutex7ReleaseEv);
stub!(_ZN16SteamThreadTools16CThreadFullMutexC1EbPKcbb);
stub!(_ZN16SteamThreadTools16CThreadFullMutexC2EbPKcbb);
stub!(_ZN16SteamThreadTools16CThreadLocalBase3SetEPv);
stub!(_ZN16SteamThreadTools16CThreadLocalBaseC1Ev);
//stub!(_ZN16SteamThreadTools16CThreadLocalBaseC2Ev);
stub!(_ZN16SteamThreadTools16CThreadLocalBaseD1Ev);
stub!(_ZN16SteamThreadTools16CThreadLocalBaseD2Ev);
stub!(_ZN16SteamThreadTools16CThreadSemaphore7ReleaseEl);
stub!(_ZN16SteamThreadTools16CThreadSemaphoreC1Ell);
stub!(_ZN16SteamThreadTools16CThreadSemaphoreC2Ell);
stub!(_ZN16SteamThreadTools17CThreadSyncObject13AssertUseableEv);
stub!(_ZN16SteamThreadTools17CThreadSyncObject14SaveNameToFileEPKc);
stub!(_ZN16SteamThreadTools17CThreadSyncObject21OpenSemaphoreInternalEPKcb);
stub!(_ZN16SteamThreadTools17CThreadSyncObject22CloseSemaphoreInternalEibPKc);
stub!(_ZN16SteamThreadTools17CThreadSyncObject23CreateSemaphoreInternalEPKclbPb);
stub!(_ZN16SteamThreadTools17CThreadSyncObject24AcquireSemaphoreInternalEj);
stub!(_ZN16SteamThreadTools17CThreadSyncObject24ReleaseSemaphoreInternalEil);
stub!(_ZN16SteamThreadTools17CThreadSyncObject27IsSemaphoreOrphanedInternalEii);
stub!(_ZN16SteamThreadTools17CThreadSyncObject29EnsureSemaphorePostedInternalEi);
stub!(_ZN16SteamThreadTools17CThreadSyncObject30EnsureSemaphoreClearedInternalEi);
stub!(_ZN16SteamThreadTools17CThreadSyncObject30SignalThreadSyncObjectInternalEv);
stub!(_ZN16SteamThreadTools17CThreadSyncObject33CreateAnonymousSyncObjectInternalEbb);
stub!(_ZN16SteamThreadTools17CThreadSyncObject4WaitEj);
stub!(_ZN16SteamThreadTools17CThreadSyncObjectC1Ev);
stub!(_ZN16SteamThreadTools17CThreadSyncObjectC2Ev);
stub!(_ZN16SteamThreadTools17CThreadSyncObjectD1Ev);
stub!(_ZN16SteamThreadTools17CThreadSyncObjectD2Ev);
stub!(_ZN16SteamThreadTools18CreateSimpleThreadEPFjPvES0_Pjj);
stub!(_ZN16SteamThreadTools7CThread10ThreadProcEPv);
stub!(_ZN16SteamThreadTools7CThread11SetPriorityEi);
stub!(_ZN16SteamThreadTools7CThread13GetThreadProcEv);
stub!(_ZN16SteamThreadTools7CThread17GetCurrentCThreadEv);
stub!(_ZN16SteamThreadTools7CThread21WaitForCreateCompleteEPNS_12CThreadEventE);
stub!(_ZN16SteamThreadTools7CThread22ThreadExceptionWrapperEPv);
stub!(_ZN16SteamThreadTools7CThread4InitEv);
stub!(_ZN16SteamThreadTools7CThread4JoinEj);
stub!(_ZN16SteamThreadTools7CThread4StopEi);
stub!(_ZN16SteamThreadTools7CThread5SleepEj);
stub!(_ZN16SteamThreadTools7CThread5StartEj);
stub!(_ZN16SteamThreadTools7CThread5YieldEv);
stub!(_ZN16SteamThreadTools7CThread6OnExitEv);
stub!(_ZN16SteamThreadTools7CThread6ResumeEv);
stub!(_ZN16SteamThreadTools7CThread7GetNameEv);
stub!(_ZN16SteamThreadTools7CThread7SetNameEPKc);
stub!(_ZN16SteamThreadTools7CThread7SuspendEv);
stub!(_ZN16SteamThreadTools7CThread9TerminateEi);
stub!(_ZN16SteamThreadTools7CThreadC1Ev);
stub!(_ZN16SteamThreadTools7CThreadC2Ev);
stub!(_ZN16SteamThreadTools7CThreadD0Ev);
stub!(_ZN16SteamThreadTools7CThreadD1Ev);
stub!(_ZN16SteamThreadTools7CThreadD2Ev);
stub!(_ZN6CTier015ValidateGlobalsER10CValidator);
stub!(_ZNK10SteamVProf9CVProfile18GetBudgetGroupNameE17EVProfBugdetGroup);
stub!(_ZNK10SteamVProf9CVProfile30BudgetGroupNameToBudgetGroupIDEPKc);
stub!(_ZNK16SteamThreadTools16CThreadFullMutex9IsCreatorEv);
stub!(_ZNK16SteamThreadTools16CThreadLocalBase3GetEv);
stub!(_ZNK16SteamThreadTools16CThreadLocalBase8BIsValidEv);
stub!(_ZNK16SteamThreadTools17CThreadSyncObjectntEv);
stub!(_ZNK16SteamThreadTools7CThread11GetPriorityEv);
stub!(_ZNK16SteamThreadTools7CThread15IsThreadRunningEv);
stub!(_ZNK16SteamThreadTools7CThread17BHasValidThreadIDEv);
stub!(_ZNK16SteamThreadTools7CThread7IsAliveEv);
stub!(_ZNK16SteamThreadTools7CThread9GetResultEv);
stub!(_ZNV16SteamThreadTools15CThreadSpinLock4LockEj);
stub!(_ZTIN16SteamThreadTools7CThreadE);
stub!(_ZTSN10SteamVProf13CVProfManagerE);
stub!(_ZTSN16SteamThreadTools7CThreadE);
stub!(_ZTVN16SteamThreadTools7CThreadE);
stub!(__bss_start);
stub!(__wrap___lxstat);
stub!(__wrap___lxstat64);
stub!(__wrap___xstat);
stub!(__wrap___xstat64);
stub!(__wrap_access);
stub!(__wrap_chmod);
stub!(__wrap_chown);
stub!(__wrap_dlmopen);
stub!(__wrap_dlopen);
stub!(__wrap_fopen);
stub!(__wrap_fopen64);
stub!(__wrap_freopen);
stub!(__wrap_lchown);
stub!(__wrap_link);
stub!(__wrap_lstat);
stub!(__wrap_lstat64);
stub!(__wrap_mkdir);
stub!(__wrap_mkfifo);
stub!(__wrap_mknod);
stub!(__wrap_mount);
stub!(__wrap_open);
stub!(__wrap_open64);
stub!(__wrap_opendir);
stub!(__wrap_rename);
stub!(__wrap_rmdir);
stub!(__wrap_scandir);
stub!(__wrap_scandir64);
stub!(__wrap_stat);
stub!(__wrap_stat64);
stub!(__wrap_statfs);
stub!(__wrap_statfs64);
stub!(__wrap_statvfs);
stub!(__wrap_statvfs64);
stub!(__wrap_symlink);
stub!(__wrap_unlink);
stub!(__wrap_utime);
stub!(__wrap_utimes);
stub!(_edata);
stub!(_end);
//stub!(_fini);
//stub!(_init);
stub!(g_ClockSpeed);
stub!(g_ClockSpeedMicrosecondsMultiplier);
stub!(g_ClockSpeedMillisecondsMultiplier);
stub!(g_ClockSpeedSecondsMultiplier);
stub!(g_VProfManager);
stub!(g_VProfProfilesRunningCount);
stub!(g_VProfile);
stub!(g_bInException);
stub!(g_pMemAllocSteam);
stub!(vtune);

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicI32, Ordering};

// tier0/threadtools.cpp
/*

undefined4 ThreadInterlockedExchange(undefined4 *param_1,undefined4 param_2)

{
  undefined4 uVar1;

  uVar1 = *param_1;
  *param_1 = param_2;
  return uVar1;
}

 */
#[no_mangle]
pub unsafe extern "C" fn ThreadInterlockedExchange(dest: *mut AtomicI32, value: i32) -> i32 {
    let result = (*dest).swap(value, Ordering::SeqCst);

    frosting::println!("(dest: {:?}, value: {:?}) -> {:?}", dest, value, result);

    result
}

// public/tier0/threadtools.h
#[derive(Debug)]
#[repr(C)]
pub struct Mutex {
    mutex: libc::pthread_mutex_t,
    attr: libc::pthread_mutexattr_t,
    current_owner_id: u32,
    lock_count: u16,
    trace: bool,
}

/*
void __thiscall SteamThreadTools::CThreadMutex::CThreadMutex(CThreadMutex *this)

{
  pthread_mutexattr_t *__attr;

  __attr = (pthread_mutexattr_t *)(this + 0x18);
  pthread_mutexattr_init(__attr);
  pthread_mutexattr_settype(__attr,1);
  pthread_mutex_init((pthread_mutex_t *)this,__attr);
  return;
}
*/
#[no_mangle]
pub unsafe extern "C" fn _ZN16SteamThreadTools12CThreadMutexC1Ev(this: *mut Mutex) {
    frosting::println!("(this: {:?})", this);

    let mut attr = MaybeUninit::uninit();

    libc::pthread_mutexattr_init(attr.as_mut_ptr());
    libc::pthread_mutexattr_settype(attr.as_mut_ptr(), libc::PTHREAD_MUTEX_RECURSIVE);

    let mut mutex = MaybeUninit::uninit();

    libc::pthread_mutex_init(mutex.as_mut_ptr(), attr.as_ptr());

    (*this).mutex = mutex.assume_init();
    (*this).attr = attr.assume_init();
}

// public/tier0/threadtools.h
#[derive(Debug)]
#[repr(C)]
pub struct ThreadLocalBase {
    key: libc::pthread_key_t,
}

// tier0/threadtools.cpp
/*
void __thiscall SteamThreadTools::CThreadLocalBase::CThreadLocalBase(CThreadLocalBase *this)

{
  code *pcVar1;
  char cVar2;
  int iVar3;

  this[4] = (CThreadLocalBase)0x0;
  iVar3 = pthread_key_create((pthread_key_t *)this,(__destr_function *)0x0);
  if (iVar3 != 0) {
    cVar2 = AssertMsgHelper<true>::AssertFailed
                      ("/data/src/tier0/threadtools.cpp",0xb47,"Out of thread local storage!\n");
    if (cVar2 == '\0') {
      pcVar1 = (code *)swi(3);
      (*pcVar1)();
      return;
    }
  }
  this[4] = (CThreadLocalBase)0x1;
  return;
}
*/
#[no_mangle]
pub unsafe extern "C" fn _ZN16SteamThreadTools16CThreadLocalBaseC2Ev(this: *mut ThreadLocalBase) {
    frosting::println!("(this: {:?})", this);

    this.byte_offset(4).cast::<u8>().write(0);

    if libc::pthread_key_create(&mut (*this).key, None) != 0 {
        panic!("out of thread local storage");
    }
    
    this.byte_offset(4).cast::<u8>().write(1);
}
