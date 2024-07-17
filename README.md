`ord`
=====

Export inscriptions in specific struct.

## Usage

### Example

get inscriptions with sequence number in range (1, 10000) from `index.redb` and export to `new_test_2`:

```bash
./ord --index=~/btc/ord/index/index.redb --cookie-file=~/bitcoincore/.cookie index export --output ~/moe/btc/ord/dump/new_test_2 --gt-sequence 1 --lt-sequence 10000
job done. 9998 recorded(cursed: 13, p2pk: 0, unbound: 0, 0-body: 1) exported in 141.470550973s. 10001 inscriptions(<= 1 included, >= 10000 not included) in block heights: [0,843911)
Percentiles distribution of inscription body size(>0), min=1, max=3915775, mean=42466.51, stdev=81483.72:
|   1.00th=[4] (samples: 46)
|   5.00th=[221] (samples: 2)
|  10.00th=[727] (samples: 3)
|  20.00th=[1471] (samples: 1196)
|  30.00th=[2287] (samples: 28)
|  40.00th=[4415] (samples: 7)
|  50.00th=[11455] (samples: 5)
|  60.00th=[25983] (samples: 8)
|  70.00th=[41215] (samples: 11)
|  80.00th=[62719] (samples: 8)
|  90.00th=[111615] (samples: 9)
|  95.00th=[199679] (samples: 6)
|  99.00th=[370687] (samples: 8)
|  99.50th=[382975] (samples: 5)
|  99.90th=[393215] (samples: 3)
|  99.95th=[397311] (samples: 4)
|  99.99th=[3915775] (samples: 1)
```

### Update Satpoint

generate utxo list by [utxo dump tool](https://github.com/in3rsha/bitcoin-utxo-dump):

```shell
bitcoin-utxo-dump -f count,txid,vout,height,coinbase,amount,script,type,address -db <chainstate_clone_path> -o <utxo_list_path>
```

generate outpoint:address mapping db by utxo list:

```shell
ord --index=~/btc/ord/index/index.redb --cookie-file=~/bitcoincore/.cookie index utxo --input ~/btc/utxo/utxo_0_852202 --output ~btc/ord/index/outpoint_addr_852202.redb --empty-address-output ~/tmp/utxo_852202_empty_address
bloom filter build cost: 26.610408463s
mapped_count: 1048576, cost: 33.50031024s
mapped_count: 2097152, cost: 40.48401334s
mapped_count: 3145728, cost: 47.598999723s
mapped_count: 4194304, cost: 54.732623976s
mapped_count: 5242880, cost: 61.920769918s
mapped_count: 6291456, cost: 69.115749233s
mapped_count: 7340032, cost: 76.300004527s
mapped_count: 8388608, cost: 83.463858458s
mapped_count: 9437184, cost: 90.701297672s
mapped_count: 10485760, cost: 97.961355307s
mapped_count: 11534336, cost: 105.217865957s
mapped_count: 12582912, cost: 112.500169259s
mapped_count: 13631488, cost: 119.80067317s
mapped_count: 14680064, cost: 127.102332702s
mapped_count: 15728640, cost: 134.373424173s
mapped_count: 16777216, cost: 141.679485419s
mapped_count: 17825792, cost: 148.951103574s
mapped_count: 18874368, cost: 156.258571182s
mapped_count: 19922944, cost: 163.570887043s
mapped_count: 20971520, cost: 170.906952788s
mapped_count: 22020096, cost: 178.226823229s
mapped_count: 23068672, cost: 185.691645063s
mapped_count: 24117248, cost: 193.106752143s
mapped_count: 25165824, cost: 200.545660261s
mapped_count: 26214400, cost: 207.911574632s
mapped_count: 27262976, cost: 215.171624057s
mapped_count: 28311552, cost: 222.484647748s
mapped_count: 29360128, cost: 229.784759636s
mapped_count: 30408704, cost: 237.082709943s
mapped_count: 31457280, cost: 244.416386079s
mapped_count: 32505856, cost: 251.724902997s
mapped_count: 33554432, cost: 259.108736225s
mapped_count: 34603008, cost: 266.430707176s
mapped_count: 35651584, cost: 273.767919612s
mapped_count: 36700160, cost: 281.052753283s
mapped_count: 37748736, cost: 288.33905365s
mapped_count: 38797312, cost: 295.681518857s
mapped_count: 39845888, cost: 302.946604253s
mapped_count: 40894464, cost: 310.270217869s
mapped_count: 41943040, cost: 317.585355255s
mapped_count: 42991616, cost: 324.937069836s
mapped_count: 44040192, cost: 332.299078974s
mapped_count: 45088768, cost: 339.668911866s
mapped_count: 46137344, cost: 347.147422738s
mapped_count: 47185920, cost: 354.667945916s
mapped_count: 48234496, cost: 362.214084086s
mapped_count: 49283072, cost: 369.734761316s
mapped_count: 50331648, cost: 377.187855319s
mapped_count: 51380224, cost: 384.720609392s
mapped_count: 52428800, cost: 392.218080816s
mapped_count: 53477376, cost: 399.699804703s
mapped_count: 54525952, cost: 407.193248311s
mapped_count: 55574528, cost: 415.112116017s
mapped_count: 56623104, cost: 422.696428613s
mapped_count: 57671680, cost: 430.405843517s
mapped_count: 58720256, cost: 438.221262987s
mapped_count: 59768832, cost: 445.923318183s
mapped_count: 60817408, cost: 453.520269744s
mapped_count: 61865984, cost: 461.108059973s
mapped_count: 62914560, cost: 468.62109547s
mapped_count: 63963136, cost: 476.38948465s
mapped_count: 65011712, cost: 488.303196833s
ord_count: 73026188， bloom_filter_positive_count: 65111901, empty_address_count: 758, mapped_count: 65111143, cost: 507.733014042s
```

update satpoint of inscriptions:

```shell
ord --index=~/btc/ord/index/index.redb --cookie-file=~/bitcoincore/.cookie index export --output ~/btc/ord/dump/ord_0_852202_new --gt-sequence 73026187 --old-output ~/btc/ord/ord_0_852202 --changes-path ~/tmp/ord_0_852202_from_old_changes --outpoint-address-map ~/btc/ord/index/outpoint_addr_852202.redb
1048576 inscriptions updated. cost: 237.622542092s
2097152 inscriptions updated. cost: 324.740527977s
3145728 inscriptions updated. cost: 372.695388817s
4194304 inscriptions updated. cost: 400.672350997s
5242880 inscriptions updated. cost: 416.86149544s
6291456 inscriptions updated. cost: 427.880470732s
7340032 inscriptions updated. cost: 437.439401734s
8388608 inscriptions updated. cost: 447.416631946s
9437184 inscriptions updated. cost: 460.222556727s
10485760 inscriptions updated. cost: 470.393145964s
11534336 inscriptions updated. cost: 484.28228896s
12582912 inscriptions updated. cost: 495.830301894s
13631488 inscriptions updated. cost: 507.849336365s
14680064 inscriptions updated. cost: 520.52015047s
15728640 inscriptions updated. cost: 531.210177208s
16777216 inscriptions updated. cost: 540.503770227s
17825792 inscriptions updated. cost: 549.803655255s
18874368 inscriptions updated. cost: 559.268274894s
19922944 inscriptions updated. cost: 567.138801158s
20971520 inscriptions updated. cost: 574.57760033s
22020096 inscriptions updated. cost: 581.889983895s
23068672 inscriptions updated. cost: 589.169520285s
24117248 inscriptions updated. cost: 596.090472531s
25165824 inscriptions updated. cost: 602.902854967s
26214400 inscriptions updated. cost: 609.732223068s
27262976 inscriptions updated. cost: 616.417413215s
28311552 inscriptions updated. cost: 623.95776267s
29360128 inscriptions updated. cost: 630.548642191s
30408704 inscriptions updated. cost: 636.726221248s
31457280 inscriptions updated. cost: 642.873686036s
32505856 inscriptions updated. cost: 649.486193311s
33554432 inscriptions updated. cost: 656.565744298s
34603008 inscriptions updated. cost: 663.205379624s
35651584 inscriptions updated. cost: 681.907996744s
36700160 inscriptions updated. cost: 689.630912616s
37748736 inscriptions updated. cost: 697.465472464s
38797312 inscriptions updated. cost: 704.019503851s
39845888 inscriptions updated. cost: 710.782429966s
40894464 inscriptions updated. cost: 717.35976226s
41943040 inscriptions updated. cost: 723.759631185s
42991616 inscriptions updated. cost: 730.918411814s
44040192 inscriptions updated. cost: 737.612148001s
45088768 inscriptions updated. cost: 745.274183424s
46137344 inscriptions updated. cost: 753.759406651s
47185920 inscriptions updated. cost: 762.972012105s
48234496 inscriptions updated. cost: 770.700050456s
49283072 inscriptions updated. cost: 778.142045796s
50331648 inscriptions updated. cost: 786.015981036s
51380224 inscriptions updated. cost: 793.598772531s
52428800 inscriptions updated. cost: 801.968801169s
53477376 inscriptions updated. cost: 810.304994653s
54525952 inscriptions updated. cost: 820.625026251s
55574528 inscriptions updated. cost: 828.129822228s
56623104 inscriptions updated. cost: 835.098337945s
57671680 inscriptions updated. cost: 843.780403992s
58720256 inscriptions updated. cost: 851.565550652s
59768832 inscriptions updated. cost: 858.80737131s
60817408 inscriptions updated. cost: 869.136671323s
61865984 inscriptions updated. cost: 885.884138651s
62914560 inscriptions updated. cost: 896.817508306s
63963136 inscriptions updated. cost: 909.545363752s
65011712 inscriptions updated. cost: 922.203111506s
66060288 inscriptions updated. cost: 933.266336701s
67108864 inscriptions updated. cost: 945.316747932s
68157440 inscriptions updated. cost: 961.714637953s
69206016 inscriptions updated. cost: 975.219181426s
70254592 inscriptions updated. cost: 987.338577109s
71303168 inscriptions updated. cost: 1005.120266231s
72351744 inscriptions updated. cost: 1020.644723912s
73026188 inscriptions updated. cost: 1027.616991953s
job done. 0 recorded(cursed: 0, unbound: 0, non-standard: 0, 0-body: 0) exported in 1027.619315327s. 0 inscriptions(<= gt_sequence included, >= lt_sequence not included) in block heights: [0,852203)
Percentiles distribution of inscription body size(>0), min=0, max=0, mean=0.00, stdev=0.00:
|   1.00th=[0] (samples: 0)
|   5.00th=[0] (samples: 0)
|  10.00th=[0] (samples: 0)
|  20.00th=[0] (samples: 0)
|  30.00th=[0] (samples: 0)
|  40.00th=[0] (samples: 0)
|  50.00th=[0] (samples: 0)
|  60.00th=[0] (samples: 0)
|  70.00th=[0] (samples: 0)
|  80.00th=[0] (samples: 0)
|  90.00th=[0] (samples: 0)
|  95.00th=[0] (samples: 0)
|  99.00th=[0] (samples: 0)
|  99.50th=[0] (samples: 0)
|  99.90th=[0] (samples: 0)
|  99.95th=[0] (samples: 0)
|  99.99th=[0] (samples: 0)```

## Pre-requisites

### Start `bitcond` server

```bash
./bitcoind -datadir=~/bitcoincore -txindex=1 -server=1
```

### Set `height` of `bitcoind`

#### Example

height 843911 block hash:

000000000000000000009fd14dd9da6a815083b2fb39d89619aeef583e094c72

set height to 843911:

```bash
./bitcoin-cli -datadir=~/bitcoincore -conf=~/bitcoincore/bitcoin.conf -rpccookiefile=~/bitcoincore/.cookie invalidateblock 000000000000000000009fd14dd9da6a815083b2fb39d89619aeef583e094c72
```

block range: [0, 843911)

## Output

```Rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InscriptionOutput {
  pub sequence_number: u32,
  pub inscription_number: i32,
  pub id: InscriptionId,
  // ord crate has different version of bitcoin dependency, using string for compatibility
  pub satpoint_outpoint: String, // txid:vout
  pub satpoint_offset: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub body: Option<Vec<u8>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_encoding: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<Vec<u8>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metaprotocol: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent: Option<Vec<InscriptionId>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pointer: Option<u64>,
  pub address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rune: Option<u128>,
}```
