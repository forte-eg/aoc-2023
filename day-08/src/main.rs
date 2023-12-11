const ARR_SZ: usize = 26 * 26 * 26;

fn main() {
    let binding = read_input::read_input().unwrap();
    let content = binding.split("\n\n").collect::<Vec<_>>();
    let [instructions, map] = content[..] else { panic!() };
    let steps = run(instructions.as_bytes(), map);
    println!("silver: {steps}");
}

fn run(instructions: &[u8], map: &str) -> u64 {
    let mut lut: [(usize, usize); ARR_SZ] = [(0, 0); ARR_SZ];
    let mut nodes = populate(map, &mut lut);

    let mut steps = 0;

    while not_done(&nodes) {
        let next_turn = steps % instructions.len();
        for node in nodes.iter_mut() {
            let new_val = match instructions[next_turn] {
                b'L' => lut[*node].0,
                b'R' => lut[*node].1,
                _ => panic!()
            };
            *node = new_val;
        }
        steps += 1;
    }
    return steps as u64;
}

fn not_done(nodes: &[usize]) -> bool {
    for node in nodes {
        if *node % (b'Z' - b'A' + 1) as usize != (b'Z' - b'A') as usize { return true }
    }
    return false
}

fn populate(data: &str, lut: &mut [(usize, usize); ARR_SZ]) -> Vec<usize> {
    let mut starting_nodes = Vec::new();
    let lines = data.split("\n").map(|s| s.as_bytes()).collect::<Vec<_>>();
    for line in lines {
        let k = &line[0..=2];
        let l = &line[7..=9];
        let r = &line[0xC..=0xE];
        let key_hash = hash(k);
        lut[key_hash] = (hash(l), hash(r));
        if k[2] == b'A' {
            starting_nodes.push(key_hash);
        }
    }
    return starting_nodes;
}

#[inline(always)]
fn hash(key: &[u8]) -> usize {
    let [a, b, c] = key else { panic!() };
    let x = b'A' as usize;
    let d = ((26 * 26) * (*a as usize - x)) + (26 * (*b as usize - x)) + (*c as usize - x);
    println!("{key:?} -> {d}");
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use read_input::read_test_input;

    #[test]
    fn ex_2() {
        let binding = read_test_input().unwrap();
        let content = binding.split("\n\n").collect::<Vec<_>>();
        let [instructions, map] = content[..] else { panic!() };
        let steps = run(instructions.as_bytes(), map);
        // assert_eq!(2, steps);
        assert_eq!(6, steps);
    }

    #[test]
    fn test_not_done1() {
        let every_finished = "AAZ ABZ ACZ ADZ AEZ AFZ AGZ AHZ AIZ AJZ AKZ ALZ AMZ ANZ AOZ APZ AQZ ARZ ASZ ATZ AUZ AVZ AWZ AXZ AYZ AZZ BAZ BBZ BCZ BDZ BEZ BFZ BGZ BHZ BIZ BJZ BKZ BLZ BMZ BNZ BOZ BPZ BQZ BRZ BSZ BTZ BUZ BVZ BWZ BXZ BYZ BZZ CAZ CBZ CCZ CDZ CEZ CFZ CGZ CHZ CIZ CJZ CKZ CLZ CMZ CNZ COZ CPZ CQZ CRZ CSZ CTZ CUZ CVZ CWZ CXZ CYZ CZZ DAZ DBZ DCZ DDZ DEZ DFZ DGZ DHZ DIZ DJZ DKZ DLZ DMZ DNZ DOZ DPZ DQZ DRZ DSZ DTZ DUZ DVZ DWZ DXZ DYZ DZZ EAZ EBZ ECZ EDZ EEZ EFZ EGZ EHZ EIZ EJZ EKZ ELZ EMZ ENZ EOZ EPZ EQZ ERZ ESZ ETZ EUZ EVZ EWZ EXZ EYZ EZZ FAZ FBZ FCZ FDZ FEZ FFZ FGZ FHZ FIZ FJZ FKZ FLZ FMZ FNZ FOZ FPZ FQZ FRZ FSZ FTZ FUZ FVZ FWZ FXZ FYZ FZZ GAZ GBZ GCZ GDZ GEZ GFZ GGZ GHZ GIZ GJZ GKZ GLZ GMZ GNZ GOZ GPZ GQZ GRZ GSZ GTZ GUZ GVZ GWZ GXZ GYZ GZZ HAZ HBZ HCZ HDZ HEZ HFZ HGZ HHZ HIZ HJZ HKZ HLZ HMZ HNZ HOZ HPZ HQZ HRZ HSZ HTZ HUZ HVZ HWZ HXZ HYZ HZZ IAZ IBZ ICZ IDZ IEZ IFZ IGZ IHZ IIZ IJZ IKZ ILZ IMZ INZ IOZ IPZ IQZ IRZ ISZ ITZ IUZ IVZ IWZ IXZ IYZ IZZ JAZ JBZ JCZ JDZ JEZ JFZ JGZ JHZ JIZ JJZ JKZ JLZ JMZ JNZ JOZ JPZ JQZ JRZ JSZ JTZ JUZ JVZ JWZ JXZ JYZ JZZ KAZ KBZ KCZ KDZ KEZ KFZ KGZ KHZ KIZ KJZ KKZ KLZ KMZ KNZ KOZ KPZ KQZ KRZ KSZ KTZ KUZ KVZ KWZ KXZ KYZ KZZ LAZ LBZ LCZ LDZ LEZ LFZ LGZ LHZ LIZ LJZ LKZ LLZ LMZ LNZ LOZ LPZ LQZ LRZ LSZ LTZ LUZ LVZ LWZ LXZ LYZ LZZ MAZ MBZ MCZ MDZ MEZ MFZ MGZ MHZ MIZ MJZ MKZ MLZ MMZ MNZ MOZ MPZ MQZ MRZ MSZ MTZ MUZ MVZ MWZ MXZ MYZ MZZ NAZ NBZ NCZ NDZ NEZ NFZ NGZ NHZ NIZ NJZ NKZ NLZ NMZ NNZ NOZ NPZ NQZ NRZ NSZ NTZ NUZ NVZ NWZ NXZ NYZ NZZ OAZ OBZ OCZ ODZ OEZ OFZ OGZ OHZ OIZ OJZ OKZ OLZ OMZ ONZ OOZ OPZ OQZ ORZ OSZ OTZ OUZ OVZ OWZ OXZ OYZ OZZ PAZ PBZ PCZ PDZ PEZ PFZ PGZ PHZ PIZ PJZ PKZ PLZ PMZ PNZ POZ PPZ PQZ PRZ PSZ PTZ PUZ PVZ PWZ PXZ PYZ PZZ QAZ QBZ QCZ QDZ QEZ QFZ QGZ QHZ QIZ QJZ QKZ QLZ QMZ QNZ QOZ QPZ QQZ QRZ QSZ QTZ QUZ QVZ QWZ QXZ QYZ QZZ RAZ RBZ RCZ RDZ REZ RFZ RGZ RHZ RIZ RJZ RKZ RLZ RMZ RNZ ROZ RPZ RQZ RRZ RSZ RTZ RUZ RVZ RWZ RXZ RYZ RZZ SAZ SBZ SCZ SDZ SEZ SFZ SGZ SHZ SIZ SJZ SKZ SLZ SMZ SNZ SOZ SPZ SQZ SRZ SSZ STZ SUZ SVZ SWZ SXZ SYZ SZZ TAZ TBZ TCZ TDZ TEZ TFZ TGZ THZ TIZ TJZ TKZ TLZ TMZ TNZ TOZ TPZ TQZ TRZ TSZ TTZ TUZ TVZ TWZ TXZ TYZ TZZ UAZ UBZ UCZ UDZ UEZ UFZ UGZ UHZ UIZ UJZ UKZ ULZ UMZ UNZ UOZ UPZ UQZ URZ USZ UTZ UUZ UVZ UWZ UXZ UYZ UZZ VAZ VBZ VCZ VDZ VEZ VFZ VGZ VHZ VIZ VJZ VKZ VLZ VMZ VNZ VOZ VPZ VQZ VRZ VSZ VTZ VUZ VVZ VWZ VXZ VYZ VZZ WAZ WBZ WCZ WDZ WEZ WFZ WGZ WHZ WIZ WJZ WKZ WLZ WMZ WNZ WOZ WPZ WQZ WRZ WSZ WTZ WUZ WVZ WWZ WXZ WYZ WZZ XAZ XBZ XCZ XDZ XEZ XFZ XGZ XHZ XIZ XJZ XKZ XLZ XMZ XNZ XOZ XPZ XQZ XRZ XSZ XTZ XUZ XVZ XWZ XXZ XYZ XZZ YAZ YBZ YCZ YDZ YEZ YFZ YGZ YHZ YIZ YJZ YKZ YLZ YMZ YNZ YOZ YPZ YQZ YRZ YSZ YTZ YUZ YVZ YWZ YXZ YYZ YZZ ZAZ ZBZ ZCZ ZDZ ZEZ ZFZ ZGZ ZHZ ZIZ ZJZ ZKZ ZLZ ZMZ ZNZ ZOZ ZPZ ZQZ ZRZ ZSZ ZTZ ZUZ ZVZ ZWZ ZXZ ZYZ ZZZ";
        let as_bytes = every_finished.split_whitespace().map(|a| a.as_bytes()).map(|a| hash(a)).collect::<Vec<_>>();
        assert_eq!(false, not_done(&as_bytes));
    }

    #[test]
    fn test_not_done2() {
        for a in b'A'..=b'Z' {
            for b in b'A'..=b'Z' {
                for c in b'A'..=b'Y' {
                    let x = [a, b, c];
                    let hash = hash(&x);
                    assert_eq!(true, not_done(&[hash]));
                }
            }
        }
    }

    #[test]
    fn test_hash() {
        let mut prev = 0;
        for a in b'A'..=b'Z' {
            for b in b'A'..=b'Z' {
                for c in b'A'..=b'Z' {
                    let x = [a, b, c];
                    let hash = hash(&x);
                    if let [b'A', b'A', b'A'] = x {assert_eq!(0, hash); continue}
                    assert_eq!(prev + 1, hash);
                    prev = hash
                }
            }
        }
    }
}