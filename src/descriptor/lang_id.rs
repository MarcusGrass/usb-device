#![allow(non_upper_case_globals, non_camel_case_types)]

#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LangID(u16);

impl From<LangID> for u16 {
    fn from(lang_id: LangID) -> Self {
        let LangID(id) = lang_id;
        id
    }
}

impl From<u16> for LangID {
    fn from(id: u16) -> Self {
        LangID(id)
    }
}

#[allow(missing_docs)]
impl LangID {
    pub const AR: LangID = LangID(0x0001);
    pub const BG: LangID = LangID(0x0002);
    pub const CA: LangID = LangID(0x0003);
    pub const ZH_HANS: LangID = LangID(0x0004);
    pub const CS: LangID = LangID(0x0005);
    pub const DA: LangID = LangID(0x0006);
    pub const DE: LangID = LangID(0x0007);
    pub const EL: LangID = LangID(0x0008);
    pub const EN: LangID = LangID(0x0009);
    pub const ES: LangID = LangID(0x000A);
    pub const FI: LangID = LangID(0x000B);
    pub const FR: LangID = LangID(0x000C);
    pub const HE: LangID = LangID(0x000D);
    pub const HU: LangID = LangID(0x000E);
    pub const IS: LangID = LangID(0x000F);
    pub const IT: LangID = LangID(0x0010);
    pub const JA: LangID = LangID(0x0011);
    pub const KO: LangID = LangID(0x0012);
    pub const NL: LangID = LangID(0x0013);
    pub const NO: LangID = LangID(0x0014);
    pub const PL: LangID = LangID(0x0015);
    pub const PT: LangID = LangID(0x0016);
    pub const RM: LangID = LangID(0x0017);
    pub const RO: LangID = LangID(0x0018);
    pub const RU: LangID = LangID(0x0019);
    pub const HR: LangID = LangID(0x001A);
    pub const SK: LangID = LangID(0x001B);
    pub const SQ: LangID = LangID(0x001C);
    pub const SV: LangID = LangID(0x001D);
    pub const TH: LangID = LangID(0x001E);
    pub const TR: LangID = LangID(0x001F);
    pub const UR: LangID = LangID(0x0020);
    pub const ID: LangID = LangID(0x0021);
    pub const UK: LangID = LangID(0x0022);
    pub const BE: LangID = LangID(0x0023);
    pub const SL: LangID = LangID(0x0024);
    pub const ET: LangID = LangID(0x0025);
    pub const LV: LangID = LangID(0x0026);
    pub const LT: LangID = LangID(0x0027);
    pub const TG: LangID = LangID(0x0028);
    pub const FA: LangID = LangID(0x0029);
    pub const VI: LangID = LangID(0x002A);
    pub const HY: LangID = LangID(0x002B);
    pub const AZ: LangID = LangID(0x002C);
    pub const EU: LangID = LangID(0x002D);
    pub const HSB: LangID = LangID(0x002E);
    pub const MK: LangID = LangID(0x002F);
    pub const ST: LangID = LangID(0x0030);
    pub const TS: LangID = LangID(0x0031);
    pub const TN: LangID = LangID(0x0032);
    pub const VE: LangID = LangID(0x0033);
    pub const XH: LangID = LangID(0x0034);
    pub const ZU: LangID = LangID(0x0035);
    pub const AF: LangID = LangID(0x0036);
    pub const KA: LangID = LangID(0x0037);
    pub const FO: LangID = LangID(0x0038);
    pub const HI: LangID = LangID(0x0039);
    pub const MT: LangID = LangID(0x003A);
    pub const SE: LangID = LangID(0x003B);
    pub const GA: LangID = LangID(0x003C);
    pub const YI: LangID = LangID(0x003D);
    pub const MS: LangID = LangID(0x003E);
    pub const KK: LangID = LangID(0x003F);
    pub const KY: LangID = LangID(0x0040);
    pub const SW: LangID = LangID(0x0041);
    pub const TK: LangID = LangID(0x0042);
    pub const UZ: LangID = LangID(0x0043);
    pub const TT: LangID = LangID(0x0044);
    pub const BN: LangID = LangID(0x0045);
    pub const PA: LangID = LangID(0x0046);
    pub const GU: LangID = LangID(0x0047);
    pub const OR: LangID = LangID(0x0048);
    pub const TA: LangID = LangID(0x0049);
    pub const TE: LangID = LangID(0x004A);
    pub const KN: LangID = LangID(0x004B);
    pub const ML: LangID = LangID(0x004C);
    pub const AS: LangID = LangID(0x004D);
    pub const MR: LangID = LangID(0x004E);
    pub const SA: LangID = LangID(0x004F);
    pub const MN: LangID = LangID(0x0050);
    pub const BO: LangID = LangID(0x0051);
    pub const CY: LangID = LangID(0x0052);
    pub const KM: LangID = LangID(0x0053);
    pub const LO: LangID = LangID(0x0054);
    pub const MY: LangID = LangID(0x0055);
    pub const GL: LangID = LangID(0x0056);
    pub const KOK: LangID = LangID(0x0057);
    pub const MNI: LangID = LangID(0x0058);
    pub const SD: LangID = LangID(0x0059);
    pub const SYR: LangID = LangID(0x005A);
    pub const SI: LangID = LangID(0x005B);
    pub const CHR: LangID = LangID(0x005C);
    pub const IU: LangID = LangID(0x005D);
    pub const AM: LangID = LangID(0x005E);
    pub const TZM: LangID = LangID(0x005F);
    pub const KS: LangID = LangID(0x0060);
    pub const NE: LangID = LangID(0x0061);
    pub const FY: LangID = LangID(0x0062);
    pub const PS: LangID = LangID(0x0063);
    pub const FIL: LangID = LangID(0x0064);
    pub const DV: LangID = LangID(0x0065);
    pub const BIN: LangID = LangID(0x0066);
    pub const FF: LangID = LangID(0x0067);
    pub const HA: LangID = LangID(0x0068);
    pub const IBB: LangID = LangID(0x0069);
    pub const YO: LangID = LangID(0x006A);
    pub const QUZ: LangID = LangID(0x006B);
    pub const NSO: LangID = LangID(0x006C);
    pub const BA: LangID = LangID(0x006D);
    pub const LB: LangID = LangID(0x006E);
    pub const KL: LangID = LangID(0x006F);
    pub const IG: LangID = LangID(0x0070);
    pub const KR: LangID = LangID(0x0071);
    pub const OM: LangID = LangID(0x0072);
    pub const TI: LangID = LangID(0x0073);
    pub const GN: LangID = LangID(0x0074);
    pub const HAW: LangID = LangID(0x0075);
    pub const LA: LangID = LangID(0x0076);
    pub const SO: LangID = LangID(0x0077);
    pub const II: LangID = LangID(0x0078);
    pub const PAP: LangID = LangID(0x0079);
    pub const ARN: LangID = LangID(0x007A);
    pub const MOH: LangID = LangID(0x007C);
    pub const BR: LangID = LangID(0x007E);
    pub const UG: LangID = LangID(0x0080);
    pub const MI: LangID = LangID(0x0081);
    pub const OC: LangID = LangID(0x0082);
    pub const CO: LangID = LangID(0x0083);
    pub const GSW: LangID = LangID(0x0084);
    pub const SAH: LangID = LangID(0x0085);
    pub const QUT: LangID = LangID(0x0086);
    pub const RW: LangID = LangID(0x0087);
    pub const WO: LangID = LangID(0x0088);
    pub const PRS: LangID = LangID(0x008C);
    pub const GD: LangID = LangID(0x0091);
    pub const KU: LangID = LangID(0x0092);
    pub const QUC: LangID = LangID(0x0093);
    pub const AR_SA: LangID = LangID(0x0401);
    pub const BG_BG: LangID = LangID(0x0402);
    pub const CA_ES: LangID = LangID(0x0403);
    pub const ZH_TW: LangID = LangID(0x0404);
    pub const CS_CZ: LangID = LangID(0x0405);
    pub const DA_DK: LangID = LangID(0x0406);
    pub const DE_DE: LangID = LangID(0x0407);
    pub const EL_GR: LangID = LangID(0x0408);
    pub const EN_US: LangID = LangID(0x0409);
    pub const ES_ES_TRADNL: LangID = LangID(0x040A);
    pub const FI_FI: LangID = LangID(0x040B);
    pub const FR_FR: LangID = LangID(0x040C);
    pub const HE_IL: LangID = LangID(0x040D);
    pub const HU_HU: LangID = LangID(0x040E);
    pub const IS_IS: LangID = LangID(0x040F);
    pub const IT_IT: LangID = LangID(0x0410);
    pub const JA_JP: LangID = LangID(0x0411);
    pub const KO_KR: LangID = LangID(0x0412);
    pub const NL_NL: LangID = LangID(0x0413);
    pub const NB_NO: LangID = LangID(0x0414);
    pub const PL_PL: LangID = LangID(0x0415);
    pub const PT_BR: LangID = LangID(0x0416);
    pub const RM_CH: LangID = LangID(0x0417);
    pub const RO_RO: LangID = LangID(0x0418);
    pub const RU_RU: LangID = LangID(0x0419);
    pub const HR_HR: LangID = LangID(0x041A);
    pub const SK_SK: LangID = LangID(0x041B);
    pub const SQ_AL: LangID = LangID(0x041C);
    pub const SV_SE: LangID = LangID(0x041D);
    pub const TH_TH: LangID = LangID(0x041E);
    pub const TR_TR: LangID = LangID(0x041F);
    pub const UR_PK: LangID = LangID(0x0420);
    pub const ID_ID: LangID = LangID(0x0421);
    pub const UK_UA: LangID = LangID(0x0422);
    pub const BE_BY: LangID = LangID(0x0423);
    pub const SL_SI: LangID = LangID(0x0424);
    pub const ET_EE: LangID = LangID(0x0425);
    pub const LV_LV: LangID = LangID(0x0426);
    pub const LT_LT: LangID = LangID(0x0427);
    pub const TG_CYRL_TJ: LangID = LangID(0x0428);
    pub const FA_IR: LangID = LangID(0x0429);
    pub const VI_VN: LangID = LangID(0x042A);
    pub const HY_AM: LangID = LangID(0x042B);
    pub const AZ_LATN_AZ: LangID = LangID(0x042C);
    pub const EU_ES: LangID = LangID(0x042D);
    pub const HSB_DE: LangID = LangID(0x042E);
    pub const MK_MK: LangID = LangID(0x042F);
    pub const ST_ZA: LangID = LangID(0x0430);
    pub const TS_ZA: LangID = LangID(0x0431);
    pub const TN_ZA: LangID = LangID(0x0432);
    pub const VE_ZA: LangID = LangID(0x0433);
    pub const XH_ZA: LangID = LangID(0x0434);
    pub const ZU_ZA: LangID = LangID(0x0435);
    pub const AF_ZA: LangID = LangID(0x0436);
    pub const KA_GE: LangID = LangID(0x0437);
    pub const FO_FO: LangID = LangID(0x0438);
    pub const HI_IN: LangID = LangID(0x0439);
    pub const MT_MT: LangID = LangID(0x043A);
    pub const SE_NO: LangID = LangID(0x043B);
    pub const YI_001: LangID = LangID(0x043D);
    pub const MS_MY: LangID = LangID(0x043E);
    pub const KK_KZ: LangID = LangID(0x043F);
    pub const KY_KG: LangID = LangID(0x0440);
    pub const SW_KE: LangID = LangID(0x0441);
    pub const TK_TM: LangID = LangID(0x0442);
    pub const UZ_LATN_UZ: LangID = LangID(0x0443);
    pub const TT_RU: LangID = LangID(0x0444);
    pub const BN_IN: LangID = LangID(0x0445);
    pub const PA_IN: LangID = LangID(0x0446);
    pub const GU_IN: LangID = LangID(0x0447);
    pub const OR_IN: LangID = LangID(0x0448);
    pub const TA_IN: LangID = LangID(0x0449);
    pub const TE_IN: LangID = LangID(0x044A);
    pub const KN_IN: LangID = LangID(0x044B);
    pub const ML_IN: LangID = LangID(0x044C);
    pub const AS_IN: LangID = LangID(0x044D);
    pub const MR_IN: LangID = LangID(0x044E);
    pub const SA_IN: LangID = LangID(0x044F);
    pub const MN_MN: LangID = LangID(0x0450);
    pub const BO_CN: LangID = LangID(0x0451);
    pub const CY_GB: LangID = LangID(0x0452);
    pub const KM_KH: LangID = LangID(0x0453);
    pub const LO_LA: LangID = LangID(0x0454);
    pub const MY_MM: LangID = LangID(0x0455);
    pub const GL_ES: LangID = LangID(0x0456);
    pub const KOK_IN: LangID = LangID(0x0457);
    pub const MNI_IN: LangID = LangID(0x0458);
    pub const SD_DEVA_IN: LangID = LangID(0x0459);
    pub const SYR_SY: LangID = LangID(0x045A);
    pub const SI_LK: LangID = LangID(0x045B);
    pub const CHR_CHER_US: LangID = LangID(0x045C);
    pub const IU_CANS_CA: LangID = LangID(0x045D);
    pub const AM_ET: LangID = LangID(0x045E);
    pub const TZM_ARAB_MA: LangID = LangID(0x045F);
    pub const KS_ARAB: LangID = LangID(0x0460);
    pub const NE_NP: LangID = LangID(0x0461);
    pub const FY_NL: LangID = LangID(0x0462);
    pub const PS_AF: LangID = LangID(0x0463);
    pub const FIL_PH: LangID = LangID(0x0464);
    pub const DV_MV: LangID = LangID(0x0465);
    pub const BIN_NG: LangID = LangID(0x0466);
    pub const FF_NG__FF_LATN_NG: LangID = LangID(0x0467);
    pub const HA_LATN_NG: LangID = LangID(0x0468);
    pub const IBB_NG: LangID = LangID(0x0469);
    pub const YO_NG: LangID = LangID(0x046A);
    pub const QUZ_BO: LangID = LangID(0x046B);
    pub const NSO_ZA: LangID = LangID(0x046C);
    pub const BA_RU: LangID = LangID(0x046D);
    pub const LB_LU: LangID = LangID(0x046E);
    pub const KL_GL: LangID = LangID(0x046F);
    pub const IG_NG: LangID = LangID(0x0470);
    pub const KR_LATN_NG: LangID = LangID(0x0471);
    pub const OM_ET: LangID = LangID(0x0472);
    pub const TI_ET: LangID = LangID(0x0473);
    pub const GN_PY: LangID = LangID(0x0474);
    pub const HAW_US: LangID = LangID(0x0475);
    pub const LA_VA: LangID = LangID(0x0476);
    pub const SO_SO: LangID = LangID(0x0477);
    pub const II_CN: LangID = LangID(0x0478);
    pub const PAP_029: LangID = LangID(0x0479);
    pub const ARN_CL: LangID = LangID(0x047A);
    pub const MOH_CA: LangID = LangID(0x047C);
    pub const BR_FR: LangID = LangID(0x047E);
    pub const UG_CN: LangID = LangID(0x0480);
    pub const MI_NZ: LangID = LangID(0x0481);
    pub const OC_FR: LangID = LangID(0x0482);
    pub const CO_FR: LangID = LangID(0x0483);
    pub const GSW_FR: LangID = LangID(0x0484);
    pub const SAH_RU: LangID = LangID(0x0485);
    pub const QUT_GT: LangID = LangID(0x0486);
    pub const RW_RW: LangID = LangID(0x0487);
    pub const WO_SN: LangID = LangID(0x0488);
    pub const PRS_AF: LangID = LangID(0x048C);
    pub const PLT_MG: LangID = LangID(0x048D);
    pub const ZH_YUE_HK: LangID = LangID(0x048E);
    pub const TDD_TALE_CN: LangID = LangID(0x048F);
    pub const KHB_TALU_CN: LangID = LangID(0x0490);
    pub const GD_GB: LangID = LangID(0x0491);
    pub const KU_ARAB_IQ: LangID = LangID(0x0492);
    pub const QUC_CO: LangID = LangID(0x0493);
    pub const QPS_PLOC: LangID = LangID(0x0501);
    pub const QPS_PLOCA: LangID = LangID(0x05FE);
    pub const AR_IQ: LangID = LangID(0x0801);
    pub const CA_ES_VALENCIA: LangID = LangID(0x0803);
    pub const ZH_CN: LangID = LangID(0x0804);
    pub const DE_CH: LangID = LangID(0x0807);
    pub const EN_GB: LangID = LangID(0x0809);
    pub const ES_MX: LangID = LangID(0x080A);
    pub const FR_BE: LangID = LangID(0x080C);
    pub const IT_CH: LangID = LangID(0x0810);
    pub const JA_PLOC_JP: LangID = LangID(0x0811);
    pub const NL_BE: LangID = LangID(0x0813);
    pub const NN_NO: LangID = LangID(0x0814);
    pub const PT_PT: LangID = LangID(0x0816);
    pub const RO_MD: LangID = LangID(0x0818);
    pub const RU_MD: LangID = LangID(0x0819);
    pub const SR_LATN_CS: LangID = LangID(0x081A);
    pub const SV_FI: LangID = LangID(0x081D);
    pub const UR_IN: LangID = LangID(0x0820);
    pub const AZ_CYRL_AZ: LangID = LangID(0x082C);
    pub const DSB_DE: LangID = LangID(0x082E);
    pub const TN_BW: LangID = LangID(0x0832);
    pub const SE_SE: LangID = LangID(0x083B);
    pub const GA_IE: LangID = LangID(0x083C);
    pub const MS_BN: LangID = LangID(0x083E);
    pub const KK_LATN_KZ: LangID = LangID(0x083F);
    pub const UZ_CYRL_UZ: LangID = LangID(0x0843);
    pub const BN_BD: LangID = LangID(0x0845);
    pub const PA_ARAB_PK: LangID = LangID(0x0846);
    pub const TA_LK: LangID = LangID(0x0849);
    pub const MN_MONG_CN: LangID = LangID(0x0850);
    pub const BO_BT: LangID = LangID(0x0851);
    pub const SD_ARAB_PK: LangID = LangID(0x0859);
    pub const IU_LATN_CA: LangID = LangID(0x085D);
    pub const TZM_LATN_DZ: LangID = LangID(0x085F);
    pub const KS_DEVA_IN: LangID = LangID(0x0860);
    pub const NE_IN: LangID = LangID(0x0861);
    pub const FF_LATN_SN: LangID = LangID(0x0867);
    pub const QUZ_EC: LangID = LangID(0x086B);
    pub const TI_ER: LangID = LangID(0x0873);
    pub const QPS_PLOCM: LangID = LangID(0x09FF);
    pub const LOCALE_CUSTOM_USER_DEFAULT: LangID = LangID(0x0C00);
    pub const AR_EG: LangID = LangID(0x0C01);
    pub const ZH_HK: LangID = LangID(0x0C04);
    pub const DE_AT: LangID = LangID(0x0C07);
    pub const EN_AU: LangID = LangID(0x0C09);
    pub const ES_ES: LangID = LangID(0x0C0A);
    pub const FR_CA: LangID = LangID(0x0C0C);
    pub const SR_CYRL_CS: LangID = LangID(0x0C1A);
    pub const SE_FI: LangID = LangID(0x0C3B);
    pub const MN_MONG_MN: LangID = LangID(0x0C50);
    pub const DZ_BT: LangID = LangID(0x0C51);
    pub const TMZ_MA: LangID = LangID(0x0C5F);
    pub const QUZ_PE: LangID = LangID(0x0C6B);
    pub const LOCALE_CUSTOM_UNSPECIFIED: LangID = LangID(0x1000);
    pub const AR_LY: LangID = LangID(0x1001);
    pub const ZH_SG: LangID = LangID(0x1004);
    pub const DE_LU: LangID = LangID(0x1007);
    pub const EN_CA: LangID = LangID(0x1009);
    pub const ES_GT: LangID = LangID(0x100A);
    pub const FR_CH: LangID = LangID(0x100C);
    pub const HR_BA: LangID = LangID(0x101A);
    pub const SMJ_NO: LangID = LangID(0x103B);
    pub const TZM_TFNG_MA: LangID = LangID(0x105F);
    pub const AR_DZ: LangID = LangID(0x1401);
    pub const ZH_MO: LangID = LangID(0x1404);
    pub const DE_LI: LangID = LangID(0x1407);
    pub const EN_NZ: LangID = LangID(0x1409);
    pub const ES_CR: LangID = LangID(0x140A);
    pub const FR_LU: LangID = LangID(0x140C);
    pub const BS_LATN_BA: LangID = LangID(0x141A);
    pub const SMJ_SE: LangID = LangID(0x143B);
    pub const AR_MA: LangID = LangID(0x1801);
    pub const EN_IE: LangID = LangID(0x1809);
    pub const ES_PA: LangID = LangID(0x180A);
    pub const FR_MC: LangID = LangID(0x180C);
    pub const SR_LATN_BA: LangID = LangID(0x181A);
    pub const SMA_NO: LangID = LangID(0x183B);
    pub const AR_TN: LangID = LangID(0x1C01);
    pub const EN_ZA: LangID = LangID(0x1C09);
    pub const ES_DO: LangID = LangID(0x1C0A);
    pub const FR_029: LangID = LangID(0x1C0C);
    pub const SR_CYRL_BA: LangID = LangID(0x1C1A);
    pub const SMA_SE: LangID = LangID(0x1C3B);
    pub const AR_OM: LangID = LangID(0x2001);
    pub const EN_JM: LangID = LangID(0x2009);
    pub const ES_VE: LangID = LangID(0x200A);
    pub const FR_RE: LangID = LangID(0x200C);
    pub const BS_CYRL_BA: LangID = LangID(0x201A);
    pub const SMS_FI: LangID = LangID(0x203B);
    pub const AR_YE: LangID = LangID(0x2401);
    pub const EN_029: LangID = LangID(0x2409);
    pub const ES_CO: LangID = LangID(0x240A);
    pub const FR_CD: LangID = LangID(0x240C);
    pub const SR_LATN_RS: LangID = LangID(0x241A);
    pub const SMN_FI: LangID = LangID(0x243B);
    pub const AR_SY: LangID = LangID(0x2801);
    pub const EN_BZ: LangID = LangID(0x2809);
    pub const ES_PE: LangID = LangID(0x280A);
    pub const FR_SN: LangID = LangID(0x280C);
    pub const SR_CYRL_RS: LangID = LangID(0x281A);
    pub const AR_JO: LangID = LangID(0x2C01);
    pub const EN_TT: LangID = LangID(0x2C09);
    pub const ES_AR: LangID = LangID(0x2C0A);
    pub const FR_CM: LangID = LangID(0x2C0C);
    pub const SR_LATN_ME: LangID = LangID(0x2C1A);
    pub const AR_LB: LangID = LangID(0x3001);
    pub const EN_ZW: LangID = LangID(0x3009);
    pub const ES_EC: LangID = LangID(0x300A);
    pub const FR_CI: LangID = LangID(0x300C);
    pub const SR_CYRL_ME: LangID = LangID(0x301A);
    pub const AR_KW: LangID = LangID(0x3401);
    pub const EN_PH: LangID = LangID(0x3409);
    pub const ES_CL: LangID = LangID(0x340A);
    pub const FR_ML: LangID = LangID(0x340C);
    pub const AR_AE: LangID = LangID(0x3801);
    pub const EN_ID: LangID = LangID(0x3809);
    pub const ES_UY: LangID = LangID(0x380A);
    pub const FR_MA: LangID = LangID(0x380C);
    pub const AR_BH: LangID = LangID(0x3C01);
    pub const EN_HK: LangID = LangID(0x3C09);
    pub const ES_PY: LangID = LangID(0x3C0A);
    pub const FR_HT: LangID = LangID(0x3C0C);
    pub const AR_QA: LangID = LangID(0x4001);
    pub const EN_IN: LangID = LangID(0x4009);
    pub const ES_BO: LangID = LangID(0x400A);
    pub const AR_PLOC_SA: LangID = LangID(0x4401);
    pub const EN_MY: LangID = LangID(0x4409);
    pub const ES_SV: LangID = LangID(0x440A);
    pub const AR_145: LangID = LangID(0x4801);
    pub const EN_SG: LangID = LangID(0x4809);
    pub const ES_HN: LangID = LangID(0x480A);
    pub const EN_AE: LangID = LangID(0x4C09);
    pub const ES_NI: LangID = LangID(0x4C0A);
    pub const EN_BH: LangID = LangID(0x5009);
    pub const ES_PR: LangID = LangID(0x500A);
    pub const EN_EG: LangID = LangID(0x5409);
    pub const ES_US: LangID = LangID(0x540A);
    pub const EN_JO: LangID = LangID(0x5809);
    pub const ES_419: LangID = LangID(0x580A);
    pub const EN_KW: LangID = LangID(0x5C09);
    pub const ES_CU: LangID = LangID(0x5C0A);
    pub const EN_TR: LangID = LangID(0x6009);
    pub const EN_YE: LangID = LangID(0x6409);
    pub const BS_CYRL: LangID = LangID(0x641A);
    pub const BS_LATN: LangID = LangID(0x681A);
    pub const SR_CYRL: LangID = LangID(0x6C1A);
    pub const SR_LATN: LangID = LangID(0x701A);
    pub const SMN: LangID = LangID(0x703B);
    pub const AZ_CYRL: LangID = LangID(0x742C);
    pub const SMS: LangID = LangID(0x743B);
    pub const ZH: LangID = LangID(0x7804);
    pub const NN: LangID = LangID(0x7814);
    pub const BS: LangID = LangID(0x781A);
    pub const AZ_LATN: LangID = LangID(0x782C);
    pub const SMA: LangID = LangID(0x783B);
    pub const KK_CYRL: LangID = LangID(0x783F);
    pub const UZ_CYRL: LangID = LangID(0x7843);
    pub const MN_CYRL: LangID = LangID(0x7850);
    pub const IU_CANS: LangID = LangID(0x785D);
    pub const TZM_TFNG: LangID = LangID(0x785F);
    pub const ZH_HANT: LangID = LangID(0x7C04);
    pub const NB: LangID = LangID(0x7C14);
    pub const SR: LangID = LangID(0x7C1A);
    pub const TG_CYRL: LangID = LangID(0x7C28);
    pub const DSB: LangID = LangID(0x7C2E);
    pub const SMJ: LangID = LangID(0x7C3B);
    pub const KK_LATN: LangID = LangID(0x7C3F);
    pub const UZ_LATN: LangID = LangID(0x7C43);
    pub const PA_ARAB: LangID = LangID(0x7C46);
    pub const MN_MONG: LangID = LangID(0x7C50);
    pub const SD_ARAB: LangID = LangID(0x7C59);
    pub const CHR_CHER: LangID = LangID(0x7C5C);
    pub const IU_LATN: LangID = LangID(0x7C5D);
    pub const TZM_LATN: LangID = LangID(0x7C5F);
    pub const FF_LATN: LangID = LangID(0x7C67);
    pub const HA_LATN: LangID = LangID(0x7C68);
    pub const KU_ARAB: LangID = LangID(0x7C92);
    pub const FR_015: LangID = LangID(0xE40C);
}
