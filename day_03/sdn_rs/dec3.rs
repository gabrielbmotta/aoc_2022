use std::str;
use std::collections::HashSet;

fn get_prio(c: char) -> u32 {
    // Really? Pulling the UNO-reverse that capitals are higher? (realistically, screw ASCII lol)
    if c >= 'a' {
        return c as u32 - 'a' as u32 + 1;
    }
    else {
        return c as u32 - 'A' as u32 + 27;
    }
}

fn get_part1(sack: &str) -> u32 {
    let mut exist = HashSet::<char>::new();
    let char_cnt = sack.chars().count();

    for (i, c) in sack.chars().enumerate() {
        if i < (char_cnt / 2) {
            exist.insert(c);
        }
        else {
            if exist.contains(&c) {
                return get_prio(c);
            }
        }
    }

    0
}

fn get_part2(sack1: &str, sack2: &str, sack3: &str) -> u32 {
    let mut exist1 = HashSet::<char>::new();
    let mut exist2 = HashSet::<char>::new();

    for c in sack1.chars()
    {
        exist1.insert(c);
    }
    for c in sack2.chars()
    {
        if exist1.contains(&c) {
            exist2.insert(c);
        }
    }

    for c in sack3.chars() {
        if exist2.contains(&c) {
            return get_prio(c);
        }
    }

    0
}

fn solve(input: &str) {
    let mut sum = 0u32;

    for line in input.split_terminator('\n') {
        if !line.is_empty() {
            sum += get_part1(line);
        }
    }
    println!("{sum}");

    sum = 0;
    let mut line1 = String::new();
    let mut line2 = String::new();
    for line in input.split_terminator('\n') {
        if !line.is_empty() {
            if      line1.is_empty() { line1 = line.to_string(); }
            else if line2.is_empty() { line2 = line.to_string(); }
            else {
                sum += get_part2(&line1, &line2, line);
                line1.clear();
                line2.clear();
            }
        }
    }
    println!("{sum}");
}


// I don't want to have to worry about dependencies or anything else,
// so just pass input as a long string. Easy to copy, not so easy to read.
fn main() {
    solve("
lvcNpRHDCnTLCJlL
RFZggsMrjTFGCJmdmd
srsBZgBqwBqRZbzqtHpzzDNtHDqV
CCTPpCvlpzzZQQQflrzbQDttTJcgcggJcHtcddtdhT
nMLBRnGsFFLznRFRLMMNBnNLDRDdhScJccctdSccJJgDDHhH
GVBGVBsLjsrrvfzpjpfQ
dzVRSPVVBVDSPzDBQVSQFFlrclMplpMJMtPJlJvHZCMt
TjmGmbhjTnTmwhmrvvrHcZvCHZMl
fnLwwqfwfqjghHwGThwfTGGBFVDFFsszSRVzRBsdBDgFsV
CCWFCcdDWwcWFpSvggnzRRQszngJwT
mGtqqLrqfmmLNtNrgTjgJzNQlvJTvznJ
tnhVbhMLLZZrnWHPSHDBWbWBFd
nQhvgnCQjSSSTTSMCsLDsfPfDlsPJMWLzL
qrqBFFBbrVRLszLfsqdqPW
bNFFRbBcFZNrZRRRbprNpFrHSwznTnvSwgHvzCSSSjnCQwgz
tnnZZVmwmqtvVdZqnddQQHHTHQLsFTnsPrrgrQ
MzMflMGpzGzPGPgjLgHrGj
zPfhMJDDMJfzlhcRJvVwcVtwVcmcbqqtbv
GVzrBVcPVfGrzVVBcQJlGGRCZSSRtSdRnGLJ
wbjvHWbLvhFppjZdtwZRNddtJwlR
pvMmbpFFbqqqvWHMFvzrLDMMrMTTrVTPzVzc
qPmgpmwpwqWWPHdjdTNStzNLMztSWtMNtz
lVFfJrFJbbcsvcRVRZzQCzQNSZTZ
DGbvFSDGbDjnqgjwmGdq
DMnpnpwwnpmRRmcRBDnDwpbRQHssHqhHCHHSsQddHZQQcqqs
JlZjjlJgNSddfsgQdf
GvrWvzNjvPVLDpbPZwBP
drQDzHsHrdZWqDSSPwmmJDDbvbSJ
hphBhCMFlBtBtGTJMJsscPwTjMJv
tlBCGFgVFNpGClFFVGGtFBZrdZQznfdQQrRWVRQdRsVf
NjdCLdjzzlNdjwBBtZqpqPJQbN
CsDWcHcGHtcBbJPpbP
mGHSssSgSsHFSgGrSgmlLzCdldllrCVCLdnfnT
rDLLzRmbcLJRtRSvSBdZtSTp
MFswshwgsCsjghgFBsGssjlZpfpvdSHfTdCZTSpHtfddTH
llwlwGjMPMQQnBMswsFgglPVcWcDcbWqLWbbLJVDzrqnVr
pqmmcSTLfSSSMFlf
rHWtPWnHtlrlDntzWwtBFdzCFMRCfjRQFfgMRMjC
PWWHDVZPDDJVlWHncGGbqqTVvVmpGTmm
wLBtWhGWJBdMmZMs
jgvNCFvvGppGnmNJ
DDRQTgcvjTPFqGHhRVhLRSVL
tPPwLpBpDpgLSPvgQCvsLPjdjNZrJZsdZjsrsnZNjbZc
lMWzWMBhmMhRGfVRffHmMjJcJjrNNZnjJcWjNqJnZJ
mFMzhTmBGfHTwgPgtptFpPgP
qCcqJQHslgtsQsCZmPWNSRNZTPBBCN
nnLpjjnvwwvDnrGwFvbFjwPgPWRTPrrPShNhmmNSRRPN
bnwdDLjnzGgvFqdJQcqVfQVqHt
DfCzDCCTDLDBCsdjzwdrHjbRgjGH
MSStMScccJtPptJNJZtJJSrFdwPGjFFHHwsggrwdwRdP
nStlpVlhhNSshZlcNZnMcctpBChWBQLqCWqmqvmBCBmQBqmL
RfLHNvfLfLZQBtRZsBfffjVqGvqpGSmJpgrJpjGjrp
FDbPCMzbTTDDPmzrVzqppJBrBj
DPcWPWDhlbCcWBsQZZfHtdwf
fbHfPfHHfPZWgZfSGpqNBqdBBjpjdPBJqv
rnVNCwwrhhDrmmvcmjdDqcmB
hFRrslFRNhFzVthllRCRCCwnQtSGfQgZZbbSWQLSSTZWbQTt
nmVqTFCmTVbnvVCnqwFrffjhZLffhNrNJF
StBHWDgMBpHMBHDzLjffjWwZJNNfNZjL
wBBcDcgzdVbbQcnQlq
MfGCtqGDhjDqHhrjGCcJZZBwHRcspZsBsHRc
PFFpLFSpzVdSTPgnzzdPPZBRcZBwBJRcWJBmJW
vdTTzVpNVpfCChMGqMvr
VtZzBzhtlrhznFlBfgrfZgFrPjGRMGjRTmSjRjRTHjfRHmRv
DQpnsbJCsNNnpNNJsDQdCDcRmHPGTHTHRSmRmHjvjHSpSS
cQbnQdNLdJJQJJJDJWnFwzgwBthrgZBwBgFLZV
VhRRgmhpFjFFBDVPGPWQPzvvMMWfjf
qcnbnCbfLqJrCnrcdbbLrGSlzWsQvsWWzvWGdMWGQl
bnfbwrcwCrHqnHcZhFBTBVRDFmpBHB
lrtqltJJJqSTWJqVHRnsRhphdbfbzBdhsRsd
vSZCgZMMLSNvCQLPLDPNgZgnznzBfsGGnQnQGdnsfhsfzb
CMFLgmPgFFNMFDDCgLLcrWrjTTjtmSJqlWTTwWSr
LdjljBdZMFdZFLLLgPvWzQRzCsCmCVssmFSW
TJttwDhnnTlWsQzSQQDvWm
HtcnfctJwtwrHhrwhfHhJpjNLMZBMgZLrBlbbLNPNj
qqhNchPdpqTTNqpDmmvvGzVfzfmvdH
cwccjsFwFjnwGwQDfVVVVv
FbWjcRsLLFngBrjpbJqCJZTbJZNClq
lhznMTSzSnjhQGtVPQBdGB
msfNDDJLWslJgfNgCrmLdtGQFVvdGQPZVttBFP
RCrJJJDrJRsfgmbsrNsrlDMTSMHcjqwzScjMqqTjbbSc
nNgsvNWDRvgnRNVCFddTNZTNZQCTFZ
lffHJfHSPmSfvLlbLmpZrCTFTtrTQHqtTrCCrq
cpzblplpbvMzWnsDDB
CgtvQvJvMtWttvwftCdWvDQrfsFcrqnlcnqZZFRcRqsnhF
HzLzVBNLjHqnhzFlWFlr
NmBjLbVVbmbTLpTjBNVLHNdCtTSWQvCgdwSwJtWQwdSD
lncHcnlccVSLNSQNslncLcrZJCrgPfJZDrggJCCvZPHC
jRqqRmmqFwRFppfPPppPBfpWBvZf
wmMqjtTdjFwGGdtNhQbVfhntcNLVbL
HFBgMjpbpddMpbHdgHLLRNwhwFLDtNSRDLLD
zsCnfqZflrlnhhrtwNgggNNL
CGqnQzlqlWWMWgVBGg
pQnvzjztpzpCmtzzjzpnBHrJNGlqggMMqgqlNWgfNNqNCP
sVTSwddRRDVShwRwRTWgPNqMGQMGNqMWslsg
hDSTQhcQcHrtcBmZHv
QRmQfvQpWpswfZWWvNbhlMglgFbZDldlbL
rzHqtcnqqVjqjGcHdLdFdCFdCbLDnMCh
DGcGGSPDpWTsSfpv
llfMHTmvHlfZlFZRzgQzsFBLtLzFGF
wrWNJrdJhRmhGmNh
DWrrJjwPjCdPDwdmwnrTZnZZcqZfnqbvZfHvql
mPmVJJmNZJmlVBPPrZpWcFWbGWbjgqNbdqjSjg
nMhzwRhwvhMDMgWHRdGHgccggd
sMvnhQshMwwvdvMMCwBtlZtplZpTmPBVZVlC
ltlRzpncRglplzhFwFwzZZMWLWZBqnDVZLDVZQQQ
SJcdvJscNSsGcSGCSJmsTQDTVZQTLTQQWCTTMQCW
sPdJmcvsJvGJmdJmfpwftfrlztrRlPfP
LdPrWcMCWCfPdMJgdFsbRRHsRSHRHHcFpH
hVVTQmQTnRFLFsmzps
qthVVwZqlQLQhNttDDDWrffDJJJDrgNP
BTjTNjtlPrBjjrljbnMFfhVWFFhlMWMfHdll
mzcgZvDggDDCJCZLvsLJLcmVqWVSVqFLfdHHMWVWWWffnF
cmmcmzQDZQJmZCnDRgQCTTwjpTtwRjrbNjpPpwrj
rMbchQphhCSbGnzSbl
qFtgvTTqFFFFJGzWJG
NZjGqGBNjNHQrhpPNHQr
LnLmbtTtTwtLcVfFFLtPrfPrfqqqsqhSvrhrhh
BzJWzZRZJzJvlZJCZgZZpJHCqQDhNQPDqDDrjNsjPPNrhSCN
lpJWZzJpgHWdWMgHlJMZzgpJLGVLGGvwVwtmcGbvGMGVvncn
WdBgdqRgWqHmNNwsGgcQ
ptPVbPbSbMJrmsVzRzhwmcGQ
SSbvrJbJtCDZfTqdRfCdBZ
WDNNWvPpvNJRRbGLsGMnnbmG
qgFdBwgVdjwdtjjdBgMgGmLQsnZrnZssGswsmLrw
FqTCTtqjdjVqgCqSMJMTvPThTJMNDh
brSSSpZjVVWdfVrHPhRBggNNGwHr
fDlLzFCLMvnMMJLNHNCBQwNhRgwRPP
MJqMmfzDvFtLDtmsVsZZTsSScWcsbq
CSZlllhSdnDrrDdJjqjzbSGzGvwbfHMb
gTNvVNLQtsFpQHqfwfBfVMfHzf
QgNmWTtmTcmmdmrZRvnlPl
SmzfvfjvjbjLNJjD
cFhWMhGHTPhccMQQGBTFGwbVVwdbddJDvVJLvDDHvd
GFMBGcWTWhcGrhFZTTchQsSfgtmnnRvnmnnRgRCrRS
ZTQHVZsZSQpQQGBMGqfBRbRB
CwtLDtNFcPnllwnqvMgbvGVfVfBG
VClLWWFPPhlhctsTrrSpWpmszjZj
HChzPltNnnHtnpqSpHpFpSfSvS
mJmQssZJLdTQLcbjlGLGfSgMbqwwSFSMSFMMqMwS
JdBLlGTjLjjjjdmmBPnRzCBRNPhPtPWPtr
FPLHMHqqPMgFLLggsMghTJhwtDSSJDltJvtwdvST
WQfmjQZsjfZNQCrZCNZQQWQBCSClvdwTSClwSwlTJvtwJdbT
BpmzrWcpBrfmpsGPGFqPRgzqVPLM
bHjccpHwGHJTfPlffPwr
VtChMZVhhStZdfTCfJvcPRCTJn
sLNLZcdNZZqZqqVqSNWtjQDGHssHGHgQHDBgmsDg
CdWgCpddwgClFlmmVTBbRtRtbntBVVds
vcJGhPLPhJvChLhMLfccrvfvsVVbsGBTTBnVbRzBVstsGnbz
vJJHhjcCLPPjQPHLrSZmpgmqwlWZZgZZQm
VpTFCFtrjCdJdjHVFnSjszSllDjsDzgvzl
fhmhZBMtfZfGBNfNcmsbZnzSlRsRggslsbnv
qPLPhMcLhPfNWPpFrdFdFTtJ
nlgQJhJFlncMzMWZMFvw
mDdsDfHjHsjHdjTLfpDsbDcNzzwcRbZNZPMcCPWMRPMc
sqqdwffHjTmdmpffmLddTTGDnSJtJBShVVhrGVJtShrlBBnJ
CrcMcMDBCmLlZdSd
qPjGjnQPqWjgZmTdlFwTmqLJ
bnPnnzHjbPznzVpdpVDcvprr
TCScMQcQCrssJPQhQs
VpfnqqfdVVwpqvqwGbDPPsjgPShDSsJhlnSl
qffdmGpfwfbfvVqpfwwfbdqRMTSTWNMWTmZLTzTCZTMLWC
QQPpPbPbDNplSJrCCj
VdMzffgnRmVdfVWRvlrCTjRlNBvrrlrr
mGgNdthhGgMWWtsFcHcHwqLqtH
HrPFVqVppVpDjFDrVbCpDFJSLsmwjhjGLLmthJLJLmZs
WWgRdMdRMnQnRzWvPSssPWssJmhsshtG
MnfvMlnQccvfMlcTRMQdRfpHPDDDpPrDTbTBNbHbHDCq
GWWRsSwLhWsRsSbsPttThZqrNBJJBgPNCJCCqNMNgP
HpVDTHzfFDpFfzHzFVcrBZCggMJBvNrNgcNNrM
jlVpVpVVQDHdFVlmmmQTlzpjjGstWLsSbsnnnStWLRhnht
prLMDDjNCLZbdFLGngdLBv
VQHmhWSSzhWHmPJRJhSmVHJPFvgTbtnTbBtGqbQnbdqgTFqG
RzwzzhwhwNCvvfpc
wQgmZnhmWVtwQmnnnQbQhzwsFcRPrFPvRJhPlPPBBBFvJv
DdjqMMGLLMMGqTGdMqdMLdBBJsJPJBJJrrBFcqRlPlqr
DsddsfMsWgfzftZb
lcqlFSFwBBPlNwPlvSlQfWsVLTQjzjWVfLsWVq
HMMbMHMtJHgFzzFrVVtfFQ
RMFpCDDFcCNBcZvP
gwDrClhppDDPwPhnmPlwDrlDjMFfMTjMTjJmRHHJBJRMJHGj
LbbZBSvSLVRHffHJHJGZ
WztdtLsSvNQStbbtzdStthWhnwnPBDclgwwnrwllCC
MnMMBppMBDWMhpnCDBgCBmRbstvPvvbGltSPVGlVPWVv
TrrddJHjNcTqrrqdFcqZwSvLSlGGPbtFRbLvFVSRPG
JTccqTcwNQcTJrZwNJcJqHwJBQDfhCBCpCQpmpDfMRfCfBpn
njVcjHfGjVjpTCpMWprW
tsSsQDvSqQshDhtmWpnQnmMmbrpdzM
FNhsDDLNLnNllBqfRJGBVHBPHRRBZZ
hFVdlFSFlfZdRhgWgdWnnnfGpMNfnLMQzQQjMD
RsrJRHsvBcvHBHjDMMpDQDjjzDHj
BcCmBqvrbbqJgmFZtWdRVSVV
FzzdDJrJCFSFRqLlwsgspsBCpL
HQdWhMZMVwqLMllw
bbQtcvZcmHtNPZcWthWRvrdrRzrSDfRSrzjJjR
bTFZzHjZNJHzLggsJgbdsWcdcShWCwsSSdvGvv
VDBmntntfCBGGGGhRc
fMnnPDfmDlmnMPmtmttnVlHzZzNLbFbLbhzJJjMgJFbb
GzgJGPRrMSgTgpgH
hcvWhBdhcfPFvmFQvwfbHMsMMbpDpTDSSHsHpd
LmcvFFlcWQlFlfPnRZPVCJzJClCz
DdCHCHrmHRgghTHH
pFVZFwfssMsgghML
tSnphvhtctSSQNDqNdmrWGvq
rqmtRmGmcWrRRQprRRnfbGMMlPGGPblwMbTP
BHHhVZSvDNdhvBVhshbzfPbTmDfnwPwzPgbl
ddvBsSSdsLdshLsLpmWqcWrCCrtFpQ
wZPCwdPCHrnLQCGZDcPRqllzqqBzjlqc
gMmgnJspsvTmWNVWNpTNWNcDcqVjqDcclhSzllRSDzqR
JnWsgMnngmttFWWMdrwCZwHfZfdfGdFd
wwgNgrsWvbfBrqqsWbjDCDDDCDCmFbSmLDLlSC
QdpdzQTVdzRMTVVzcHTQLnlFmZHPSChCmlDPPHnP
dRMLVttzzVtTVQVqrrrgBtsvWWwtNw
vtBvntlqMvfnTfPDPhdRNbhdTFzF
QLWcmrrcgmCgCcsgcQWlWWrrDjjzzjsdFDdRhPNDhhhzDzPF
GGWHcCQcCCSlmmBVMGVBfMqwJtqv
cfqfhDRwhqZgRgRzRvcfhBSrsBnrDBBJWrnrWrSmmr
VCTVjGCTCjFddQntmrsVsJvrtrrW
PFQGpFbvPRMNqgRq
MmDgZZGMjZGfZRFztzCtCjzSrF
cBNpPJpBdNntcBHBccJlsSVVzzSwlCRrCnzsFw
PPBJLPPBBLPHBNQgqfMQtmTftGGvhq
bbZnbnVVgVSnbgZtntZrltsprpMCJvpqdJmsCMMmvvCq
BjDcjLLDzNjLDcjDzhcDNLLLHdpmpHJsqsMMNfCHfJpspqvp
dFLTFBcBzjFLgTbQtRgTVTlZ
nqNnrBRjLnjLZCqGGlqSGWlWDS
mTJTTcTJJfJfhhhwMbQDPWCQFCRlbDCSDDPl
dhMcRgJmgRrBrrrNgrLZ
GvJvJSGZFrGmmbmCrWnhjncLctcWttVqjLBB
wDlTzwlHTncRTqnRBt
gspglgzDzdPDfpgfdDzsgMPGvZJBrrbZGJNFmCFvmFvFvM
RLjMZZzfvNLBdjQfBfQdhRfSTVlcVqGbGcLGlbmqLVccmm
FggHCwsggrWWtCHJDDHtWrTNNlqSnlTlnGVTmWGcbcbm
PtpttrwsJssPsdRQvphZNzdMBh
NqqpZBHqTBpPNpPpGwwMPGTJjjLjQljGmtLfftllbJQfGf
nHczcrSFnVWSlrltrgJmjLLQ
SvzcDDVVFzdzhndCFSvnhcDspRDRDMPpMHRNPPZqppwM
FRSbVCSFFCDMFjRMjSSVFSWggMmWtWngJWttWmmJctnt
BPwcQQcQqQmWHfgrfwrh
PPlBQdNQvdLzzvclczdNRSbpLSbRbDjFZFFZVsFs
wtrrVhBbpcZSSjBfSfmm
MDWTvTMGMRCDCTQWsvfrRjjFfHlFmjlmLlHl
gQrTQvQDssdNWGsTstcbptwVPqcbpNqttP
jtGSwGQczrzjtGzrcsJwMRqMVMwRVMWFvVTWFV
DhLgnDLndDHmLvWqpTHqHHVNqF
LhdmPhfgZnZDlPCPmDfljQtGsJtBsWjGJzJSWBjC
zHDjcjBjTfjjfGpf
NNFTnNwPNNdqnJdFnqqTgmgftfftrWCZGbmrpWttmW
FLJqVNVNhnwnTRsRQBlQzShs
HJGJGJzzHHQHfJHsnNsGMbccMrTgbr
vddSCCjdmVvDDmvmBVbbBchcrrcscMTTnscn
VjWdFCVVMWWmjdSVFSVpqwwZttfJJltJZqltppLw
SnmPBPBnMLnPBsSgSDqRNRRccDfNcNQQRg
lZVWtWVzCjvZzCCGzDwbwRwtqJwJNTtDfD
zCzHZFFFfdLnBfFf
NRBFpNNJgNbWbJLRpRbWNtNpZllCZdjjZfjPVljTVCZQltlV
sDqHmsHcDrwHhMDlfCQfVBjDPCTd
MnGGcqwhhsrchcmGpzRJSSGGJWJzbJBR
LBzjQQzcjWvHWLnVDdnHRffHDCVR
rmJSrPJJsbNZssGSPrFpddfGwDRRpVjVpCGdCp
mPsrmsNTrPNLTjQlWQhqLc
MSDFszbhbRRTRdwhtw
PWmCZCmZVvGqMcjmJRpdTTtdLpqwdlll
MWGmmCVHvMSBDSNbbHHS
mBwSBSfSPHZCLPZSWwfPppTndVpdVncFgcgPpP
rhQJjzQjrltJzGqCrGJTvgqRpnTgFgcFTVFqFR
QbhGMJrhzrhQGQCsjwMDNWBDSZBZwSBLmM
bQfDPgDQbQNGPgflWfvMZcRMMFmcvMfZ
BLqSssjnzpBwszqwzFCNMvzWvRvzCCZFvc
BpNpjnNHnSpssqLqrBLLHjdhTPPDgbggllldhTPdrrQh
CvCMqNWVVqqPvNvvChhhdSnFHwBdWwhfdS
gqTZGGjlmclrZjlmSndSDwfFhhDBHm
tRrZpgrcctbbltRpgtqCVJCvPsRvsvPCQPCLPz
dTjRdWDBRzvjfzTfvTJPtJttsSLqHsSQJw
hrJNmbnFNZrbhlCsqltqcQcQSQqwPL
pVNhFgZphZmjzvjGDJWzVJ
gWzQhCWbQnCCFgCJnFQnWCzwjrHjjHGTwHGrhLwjjjtStL
splcpqDNDqqcZqRlspwHbjVrjjHTrwSbtVNV
BpMslDqDmRDRsBRBJPPnbzCfvQgmCWJb
tRtgRQWCwlTglHZHTglCtTdbbfvhWpbSBbhWzzbfGpfhbb
cqZVMJmLqmNrsJMDbzGrGSvzGBhvvG
mmnJPMZcclFRdnQtCQ
QVQVqfFzVVQQrQwZsCTrBtTrccTtctcJRRjT
vNNPnvGbBtWBLvBf
mMHbfDfHdHGmnhDDqZFDzVSQzF
NNlTNFCRTrfllTZsPWSsFPfzJdVQVpDQVszQVtpbzJMVbJ
LNHjNHjmLLjNqvGgvVQJQDVLbDVDpdQQzQ
nqmqGHjwgHvgwGHjGgccNTSWrrlCZrFfPSFFCP
qWzCQqhPCHjHmqJhqvqmjRgSFMTFggMFTFVRVVTgTm
SptGsDlnGfnDLgTMTwgRFFFs
bBcntZdpGZZcctlGtDfnDnBCSWqJvQhqjqzjhJqJQCQWPd
SjZJrSSDShddqLvPqzzdwq
nTssfRpQQmQCHlPBBgGmwVGzwm
TWQsbCRQFHFWQRTpzRHRsRrMtDrjhjbtMbccrttJjJht
cCChVMwPPMHCPCCPrvJnntdTJSvTtdrSRt
FGfFDBhGGlfGGfWJWdbSRSnRNbTvdn
fGpGlDmBhflgfDFmmfFpcVzMzqZZzcCPQVZzqP
SmgtSjGPjppBjbqqWTCZDQTHHHTg
VsFfCzLvsMfzNfNRhVMslzlHqrWrQDQcqDqTrLWHcrWJJW
dsRdsNCvNMVpwPdnnGbbPb
");
}