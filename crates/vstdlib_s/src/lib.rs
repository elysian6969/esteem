#![allow(unused_variables)]

use core::str::FromStr;
use esteem_util::stub;

stub!(BHanIdeograph);
stub!(CanBeginLine);
stub!(CanBreakBetween);
stub!(CanBreakRepeated);
stub!(CanEndLine);
stub!(Coroutine_Cancel);
stub!(Coroutine_Continue);
stub!(Coroutine_Create);
stub!(Coroutine_DebugBreak);
stub!(Coroutine_GetCurrentlyActive);
stub!(Coroutine_GetStackDepth);
stub!(Coroutine_IsActive);
stub!(Coroutine_IsAddressInOurStack);
stub!(Coroutine_ReleaseThreadMemory);
stub!(Coroutine_Test);
stub!(Coroutine_ValidateGlobals);
stub!(Coroutine_YieldToMain);
stub!(CreateInterface);
stub!(DebugStatsSystem);
stub!(GetNameFromOSType);
stub!(GetOSDetailString);
stub!(GetOSType);
stub!(GetPlatformFromOS);
stub!(GetPlatformName);
stub!(GetPlatformNameFromEPlatformType);
stub!(GetPortableOsVersionInformation);
stub!(InstallUniformRandomStream);
stub!(KeyValuesSystemSteam);
stub!(OSIsCompatible);
stub!(OSTypesAreCompatible);
stub!(SteamRealPath);
stub!(StringAfterPrefix);
stub!(StringAfterPrefixCaseSensitive);
stub!(StringFindSuffix);
stub!(VStdLib_GetICVarFactory);
stub!(V_AggressiveStripPrecedingAndTrailingWhitespace);
stub!(V_AggressiveStripPrecedingAndTrailingWhitespaceW);
stub!(V_AppendParamToURL);
stub!(V_AppendSlash);
stub!(V_BasicHtmlEntityEncode);
stub!(V_BasicHtmlEntityEncodeBuffer);
stub!(V_CP437CharsToUTF8);
stub!(V_ComposeFileName);
stub!(V_CopyAndFixSlashes);
stub!(V_DefaultExtension);
stub!(V_EscapeStringForPHP);
stub!(V_ExtractDomainFromURL);
stub!(V_ExtractFileExtension);
stub!(V_ExtractFilePath);
stub!(V_FileBase);
stub!(V_FixDoubleSlashes);
stub!(V_FixSlashes);
stub!(V_FixupPathName);
stub!(V_FormatAndAppend);
stub!(V_FormatAndAppendTail);
stub!(V_FormatAndAppendV);
stub!(V_Get2LDFromDomainName);
stub!(V_GetCurrentDirectory);
stub!(V_GetFileExtension);
stub!(V_HtmlEntityDecodeToUTF8);
stub!(V_IsAbsolutePath);
stub!(V_IsDeprecatedW);
stub!(V_IsMeanSpaceW);
stub!(V_IsSteamCloseProtocol);
stub!(V_IsTLD);
stub!(V_IsValidDomainNameCharacter);
stub!(V_IsValidUChar32);
stub!(V_IsValidURLCharacter);
stub!(V_JoinNumbers);
stub!(V_JoinNumbersUint);
stub!(V_JoinNumbersUint64);
stub!(V_MakeAbsolutePath);
stub!(V_MakeFileSafeName);
stub!(V_MakeRelativePath);
stub!(V_MatchWildcardDNSName);
stub!(V_MatchWildcardString);
stub!(V_NormalizeUTF8);
stub!(V_NormalizeUTF8Old);
stub!(V_ParseURLQueryString);
stub!(V_RemoveChar);
stub!(V_RemoveDotSlashes);
stub!(V_ReplaceBadFilenameCharacters);
stub!(V_ReplaceBadFilenameCharactersInPlace);
stub!(V_SetCurrentDirectory);
stub!(V_SetExtension);
stub!(V_SetURLParam);
stub!(V_SplitNumbers);
stub!(V_SplitNumbersUint);
stub!(V_SplitStringInternal);
stub!(V_StrLeft);
stub!(V_StrReplaceChar);
stub!(V_StrRight);
stub!(V_StrSkipArticles);
stub!(V_StrSlice);
stub!(V_StrSubst);
stub!(V_StrSubstInPlace);
stub!(V_StrTrim);
stub!(V_StripAndPreserveHTML);
stub!(V_StripAndPreserveHTMLCore);
stub!(V_StripBBCode);
stub!(V_StripExtension);
stub!(V_StripFilename);
stub!(V_StripLastDir);
stub!(V_StripPrecedingAndTrailingWhitespace);
stub!(V_StripPrecedingAndTrailingWhitespaceW);
stub!(V_StripTrailingSlash);
stub!(V_StripTrailingWhitespaceASCII);
stub!(V_StripUnprintable);
stub!(V_StripUnprintableW);
stub!(V_TruncateAbsolutePathToDiskIdentity);
stub!(V_UChar32ToUTF16);
stub!(V_UChar32ToUTF16Len);
stub!(V_UChar32ToUTF8);
stub!(V_UChar32ToUTF8Len);
stub!(V_URLContainsDomain);
stub!(V_URLCracker);
stub!(V_URLDecode);
stub!(V_URLDecodeRaw);
stub!(V_URLEncode);
stub!(V_URLEncodeRaw);
stub!(V_URLParse);
stub!(V_URLPartsToString);
stub!(V_UTF16ToUChar32);
stub!(V_UTF16ToUTF32);
stub!(V_UTF16ToUTF8);
stub!(V_UTF32CharsToUTF16);
stub!(V_UTF32CharsToUTF8);
stub!(V_UTF32ToUTF16);
stub!(V_UTF32ToUTF8);
stub!(V_UTF8ToUChar32);
stub!(V_UTF8ToUTF16);
stub!(V_UTF8ToUTF32);
stub!(V_UnqualifiedFileName);
stub!(V_WcsSkipArticles);
stub!(V_Windows1252CharsToUTF8);
//stub!(V_atof);
stub!(V_atoi);
//stub!(V_atoui64);
stub!(V_binarytohex);
stub!(V_hextobinary);
stub!(V_isbreakablewspace);
stub!(V_isbreakablewspace32);
stub!(V_isnumeric);
stub!(V_isstrlower);
stub!(V_isvalidhex);
stub!(V_iswalpha32);
stub!(V_memcpy_nocache);
stub!(V_pretifymem);
stub!(V_pretifynum);
stub!(V_snprintf);
stub!(V_strcat);
stub!(V_strcmp_prefix);
stub!(V_stricmp_filename);
stub!(V_stricmp_prefix);
stub!(V_strlower_fast);
stub!(V_strnappend);
stub!(V_strncat);
stub!(V_strncat_length);
stub!(V_strncat_length_unbounded);
stub!(V_strnchr);
stub!(V_strncmp);
stub!(V_strncpy);
stub!(V_strnicmp);
stub!(V_strnistr);
stub!(V_strnlen);
stub!(V_strristr);
stub!(V_strtoi64);
stub!(V_strtoui64);
stub!(V_strupper_fast);
stub!(V_tolower);
stub!(V_toupper);
stub!(V_towlower32);
stub!(V_towupper32);
stub!(V_vsnprintf);
stub!(V_vsnprintfRet);
stub!(V_vsnwprintf);
stub!(V_wcscat);
stub!(V_wcsncat);
stub!(V_wcsncpy);
stub!(V_wcstoi64);
stub!(V_wcstoui64);
stub!(V_whextobinary);
stub!(WeakRandomChar);
stub!(WeakRandomFillMemory);
stub!(WeakRandomFloat);
stub!(WeakRandomGaussianFloat);
stub!(WeakRandomInt);
stub!(WeakRandomSeed);
stub!(WeakRandomUint32);
stub!(_Z10V_atoui128PKc);
stub!(_Z14V_SplitString3PKcPKS0_iR10CUtlVectorI10CUtlString10CUtlMemoryIS4_EES8_);
stub!(_Z15V_UnicodeLengthPKc);
stub!(_Z15V_UnicodeLengthPKt);
stub!(_Z15V_UnicodeLengthPKw);
stub!(_Z15V_UnicodeRepairPc25EStringConvertErrorPolicy);
stub!(_Z15V_UnicodeRepairPt25EStringConvertErrorPolicy);
stub!(_Z15V_UnicodeRepairPw25EStringConvertErrorPolicy);
stub!(_Z16V_UnicodeAdvancePci);
stub!(_Z16V_UnicodeAdvancePti);
stub!(_Z16V_UnicodeAdvancePwi);
stub!(_Z17V_UnicodeValidatePKc);
stub!(_Z17V_UnicodeValidatePKt);
stub!(_Z17V_UnicodeValidatePKw);
stub!(_Z20V_UnicodeCaseConvertPKcPcii25EStringConvertErrorPolicy);
stub!(_Z20V_UnicodeCaseConvertPKtPtii25EStringConvertErrorPolicy);
stub!(_Z20V_UnicodeCaseConvertPKwPwii25EStringConvertErrorPolicy);
stub!(_Z20V_UnicodeCaseConvertR10CUtlStringi25EStringConvertErrorPolicy);
stub!(_Z20V_UnicodeCaseConvertR10CUtlVectorIw10CUtlMemoryIwEEi25EStringConvertErrorPolicy);
stub!(_Z20V_UnicodeCaseConvertR17CUtlStringBuilderi25EStringConvertErrorPolicy);
stub!(_Z21V_ConvertStringToUUIDPKc16EUUIDStringFlagsP6V_UUID);
stub!(_Z21V_ConvertStringToUUIDPKw16EUUIDStringFlagsP6V_UUID);
stub!(_Z21V_ConvertUUIDToStringPK6V_UUID16EUUIDStringFlagsPci);
stub!(_Z21V_ConvertUUIDToStringPK6V_UUID16EUUIDStringFlagsPwi);
stub!(_Z21V_SplitStringInternalPKcPKS0_PS0_iR10CUtlVectorI10CUtlString10CUtlMemoryIS5_EEb);
stub!(_Z23V_ParseShellCommandLinePKcR10CUtlVectorI10CUtlString10CUtlMemoryIS2_EEiPS0_);
stub!(_Z28V_ParseShellCommandLinePOSIXPKcR10CUtlVectorI10CUtlString10CUtlMemoryIS2_EEiPS0_);
stub!(_Z28V_ParseShellCommandLineWin32PKcR10CUtlVectorI10CUtlString10CUtlMemoryIS2_EEiPS0_);
stub!(_Z30V_EscapeShellArgumentAndAppendP17CUtlStringBuilderPKc);
stub!(_Z30V_EscapeShellArgumentAndAppendPcjPKc);
stub!(_Z35V_EscapeShellArgumentAndAppendPOSIXPcjPKc);
stub!(_Z35V_EscapeShellArgumentAndAppendWin32PcjPKc);
stub!(_Z9V_stristrPKcS0_);
stub!(_ZN11SteamStdLib11CommandLineEv);
//stub!(_ZN11SteamStdLib17CCommandLineParamC1EPKcS2_);
//stub!(_ZN11SteamStdLib17CCommandLineParamC2EPKcS2_);
stub!(_ZN11SteamStdLib17CCommandLineParamD1Ev);
stub!(_ZN11SteamStdLib17CCommandLineParamD2Ev);
stub!(_ZN20CStringNormalization10InitializeEv);
stub!(_ZN20CStringNormalization15ValidateStaticsER10CValidatorPKc);
stub!(_ZN20CStringNormalization24Test_BValidateStructuresEv);
stub!(_ZN20CStringNormalization4FoldEPKwPwi);
stub!(_ZN20CStringNormalization9NormalizeEbPKcPci);
stub!(_ZN20CStringNormalization9NormalizeEbPKwPwi);
stub!(_ZN20CUniformRandomStream10RandomCharEv);
stub!(_ZN20CUniformRandomStream11RandomFloatEff);
stub!(_ZN20CUniformRandomStream16RandomFillMemoryEPvj);
stub!(_ZN20CUniformRandomStream20GenerateRandomNumberEv);
stub!(_ZN20CUniformRandomStream7SetSeedEi);
stub!(_ZN20CUniformRandomStream9RandomIntEii);
stub!(_ZN20CUniformRandomStreamC1Ev);
stub!(_ZN20CUniformRandomStreamC2Ev);
stub!(_ZN21CGaussianRandomStream11RandomFloatEff);
stub!(_ZN21CGaussianRandomStream14AttachToStreamEP20IUniformRandomStream);
stub!(_ZN21CGaussianRandomStreamC1EP20IUniformRandomStream);
stub!(_ZN21CGaussianRandomStreamC2EP20IUniformRandomStream);
stub!(_ZN9CURLUtils15ValidateStaticsER10CValidatorPKc);
stub!(_ZN9CURLUtils20FixupSeparatorSchemeER10CUtlString);
stub!(_ZN9CURLUtils5IsTLDEPKc);
stub!(_ZTI20CUniformRandomStream);
stub!(_ZTS20CUniformRandomStream);
stub!(_ZTV20CUniformRandomStream);
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
stub!(_strlwr);
stub!(_strupr);
stub!(vstdlib_wcsicmp);
stub!(vstdlib_wcsnicmp);
stub!(vstdlib_wcstoi64);
stub!(vstdlib_wcstoui64);

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/public/tier1/strtools.h
#[no_mangle]
pub unsafe extern "C" fn V_atof(string: *const u8) -> f32 {
    let string = esteem_util::str_from_ptr(string);
    let result = f64::from_str(string)
        .map(|value| value as f32)
        .unwrap_or(f32::NAN);

    frosting::println!("(string: {:?}) -> {:?}", string, result);

    result
}

#[no_mangle]
pub unsafe extern "C" fn V_atoui64(string: *const u8) -> u64 {
    let string = esteem_util::str_from_ptr(string);
    let result = if string.starts_with("0x") {
        u64::from_str_radix(&string[2..], 16).ok()
    } else {
        None
    }
    .or_else(|| Some(f32::from_str(string).ok()? as u64))
    .or_else(|| string.parse().ok())
    .unwrap_or(u64::MAX);

    frosting::println!("(string: {:?}) -> {:?}", string, result);

    result
}

// SteamStdLib::CCommandLineParam::CCommandLineParam(char const*, char const*)
#[no_mangle]
pub unsafe extern "C" fn _ZN11SteamStdLib17CCommandLineParamC1EPKcS2_(
    this: *const (),
    param: *const u8,
) -> bool {
    let param = esteem_util::str_from_ptr(param);

    frosting::println!("(this: {:?}, param: {:?})", this, param);

    false
}

// SteamStdLib::CCommandLineParam::CCommandLineParam(char const*, char const*)
#[no_mangle]
pub unsafe extern "C" fn _ZN11SteamStdLib17CCommandLineParamC2EPKcS2_(
    this: *const (),
    param: *const u8,
) -> bool {
    let param = esteem_util::str_from_ptr(param);

    frosting::println!("(this: {:?}, param: {:?})", this, param);

    false
}
