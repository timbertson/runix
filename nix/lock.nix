final:
let
  pkgs = final.pkgs;
in
{
  context = {
    type = "cargo";
    version = "1";
    root = "_virtual_root";
  };
  specs = {
    _virtual_root = {
      pname = "fetlock-virtual-root";
      version = "dev";
      depKeys = [
        ("runix-0.1.0")
      ];
      src = (final.pathSrc ../.);
    };
    "aho-corasick-0.7.18" = {
      pname = "aho-corasick";
      version = "0.7.18";
      depKeys = [
        ("memchr-2.5.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-HjfP1edletpF90LW6ZyleIWAtcUp3Hj68R7ObccCZW8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/aho-corasick/0.7.18/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "anyhow-1.0.62" = {
      pname = "anyhow";
      version = "1.0.62";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-FIXU0sxF57IB7jdnAVyW+qWQQ4fJ2Hxu/dD7UR8S0wU=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/anyhow/1.0.62/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "atty-0.2.14" = {
      pname = "atty";
      version = "0.2.14";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-2bOb4Ydw0RQhzbG5lHpF3T836TCSy/N3YUgooxnV/ug=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download";
      });
      edition = "2015";
    };
    "autocfg-1.1.0" = {
      pname = "autocfg";
      version = "1.1.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-1GiAK6sXy8DMV16bBT9B5yqja/prf1XjUp/6QxYbl/o=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/autocfg/1.1.0/download";
      });
      edition = "2015";
    };
    "base64-0.13.0" = {
      pname = "base64";
      version = "0.13.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-kE3+rFDzzauij8b1f9zdt19J7WE0ZnanjE/+VYd4Av0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/base64/0.13.0/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "bitflags-1.3.2" = {
      pname = "bitflags";
      version = "1.3.2";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-vvONRRY8Lx3eCUp9/TPM9ZXJKQXI+PT9wY0G+xA3cYo=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/bitflags/1.3.2/download";
      });
      edition = "2018";
      features = [
        ("default")
      ];
    };
    "bumpalo-3.11.0" = {
      pname = "bumpalo";
      version = "3.11.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-wa2CIRjSDSwjT0JwANWsw26r4eKaNIyJtj3WCxPyjl0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/bumpalo/3.11.0/download";
      });
      edition = "2021";
      features = [
        ("default")
      ];
    };
    "bytes-1.2.1" = {
      pname = "bytes";
      version = "1.2.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-7Ip7anD96ANyFUxlcC8AoPVvPhw2q7xsRASEviSIVts=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/bytes/1.2.1/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "cc-1.0.73" = {
      pname = "cc";
      version = "1.0.73";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-L/8qaSezu4f5WV1nGWpwST9idoenHYeg1pIkLDP1jBE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/cc/1.0.73/download";
      });
      crateBin = [
        ({
          name = "gcc-shim";
          path = "src/bin/gcc-shim.rs";
        })
      ];
      edition = "2018";
    };
    "cfg-if-1.0.0" = {
      pname = "cfg-if";
      version = "1.0.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-uvHeQzl2FYi8Bhnjy8ASDuWC67dLU7Tvv3kRe9LaQP0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download";
      });
      edition = "2018";
    };
    "core-foundation-0.9.3" = {
      pname = "core-foundation";
      version = "0.9.3";
      depKeys = [
        ("core-foundation-sys-0.8.3")
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-GUp6nm3lP6VRFpNAZ8hE2ddJMS91xvbQmA6MJS+MIUY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/core-foundation/0.9.3/download";
      });
      edition = "2015";
    };
    "core-foundation-sys-0.8.3" = {
      pname = "core-foundation-sys";
      version = "0.8.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-WCfOv0ZwRouHct0ZGFZ2iu3LGwJ4oE+Yn3dmNRkXudw=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.8.3/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
    };
    "either-1.8.0" = {
      pname = "either";
      version = "1.8.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-kOXByDaIAxE78MlYT8SVpYuG3Iop7b+P6HfSHZUH55c=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/either/1.8.0/download";
      });
      edition = "2018";
    };
    "encoding_rs-0.8.31" = {
      pname = "encoding_rs";
      version = "0.8.31";
      depKeys = [
        ("cfg-if-1.0.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-mFJjVYncn56htv6fBbUO8gjIXINKVi8MarscR1c27Cs=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/encoding_rs/0.8.31/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("alloc")
        ("default")
      ];
    };
    "env_logger-0.9.0" = {
      pname = "env_logger";
      version = "0.9.0";
      depKeys = [
        ("atty-0.2.14")
        ("humantime-2.1.0")
        ("log-0.4.17")
        ("regex-1.6.0")
        ("termcolor-1.1.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-CyzwNElx7mxkwxvg1TB5P7pFfTIt/sKBDEU9DvIo+cM=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/env_logger/0.9.0/download";
      });
      edition = "2018";
      features = [
        ("atty")
        ("default")
        ("humantime")
        ("regex")
        ("termcolor")
      ];
    };
    "fastrand-1.8.0" = {
      pname = "fastrand";
      version = "1.8.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-p6QHz6ozhcSuayPoRiPUjCeY0G4+ahh49/WfF7P4ZJk=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/fastrand/1.8.0/download";
      });
      edition = "2018";
    };
    "fnv-1.0.7" = {
      pname = "fnv";
      version = "1.0.7";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-P57skY0/JAad7LmvFVTK18iA4tokqa/YisoABTGrgsE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/fnv/1.0.7/download";
      });
      edition = "2015";
      features = [
        ("default")
        ("std")
      ];
      libPath = "lib.rs";
    };
    "foreign-types-0.3.2" = {
      pname = "foreign-types";
      version = "0.3.2";
      depKeys = [
        ("foreign-types-shared-0.1.1")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-9vM564rcBSzSyniRD9qGmu+jjSLVy2SOZIXk0/wG87E=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/foreign-types/0.3.2/download";
      });
      edition = "2015";
    };
    "foreign-types-shared-0.1.1" = {
      pname = "foreign-types-shared";
      version = "0.1.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-ALAihBGQjKhoXbp/ws3XDsmZCm51Pom2rJGoTED7r0s=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/foreign-types-shared/0.1.1/download";
      });
      edition = "2015";
    };
    "form_urlencoded-1.0.1" = {
      pname = "form_urlencoded";
      version = "1.0.1";
      depKeys = [
        ("matches-0.1.9")
        ("percent-encoding-2.1.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-X8Jah/pP0glL/7BpJYUgNNkKF/DR4FGX1JVtNVV1IZE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/form_urlencoded/1.0.1/download";
      });
      edition = "2018";
    };
    "futures-channel-0.3.23" = {
      pname = "futures-channel";
      version = "0.3.23";
      depKeys = [
        ("futures-core-0.3.23")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-K/xSy93P10W/F0AzhJK7C9g9dsZ7RF+Rxfsp+uKeyqE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-channel/0.3.23/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("alloc")
        ("default")
        ("std")
      ];
    };
    "futures-core-0.3.23" = {
      pname = "futures-core";
      version = "0.3.23";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-0qztrojTgjWTbDkiR2sQ/O17K2gTb148A8LVvjSKERU=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-core/0.3.23/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("alloc")
        ("default")
        ("std")
      ];
    };
    "futures-io-0.3.23" = {
      pname = "futures-io";
      version = "0.3.23";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-k6ZvxtA1omo64lWm0ryjXtpjrkxVEr71REkRP3oSKOU=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-io/0.3.23/download";
      });
      edition = "2018";
      features = [
        ("std")
      ];
    };
    "futures-sink-0.3.23" = {
      pname = "futures-sink";
      version = "0.3.23";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-yguuH+l1LPf9mwBkxnSuY/l7N7xxTXRcveCvt+xOZ2U=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-sink/0.3.23/download";
      });
      edition = "2018";
      features = [
        ("alloc")
        ("default")
        ("std")
      ];
    };
    "futures-task-0.3.23" = {
      pname = "futures-task";
      version = "0.3.23";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-hC/GO5MfQFaiTVneE/sSchNM4mGBbgY+Y0rQwVzcUwY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-task/0.3.23/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("alloc")
        ("std")
      ];
    };
    "futures-util-0.3.23" = {
      pname = "futures-util";
      version = "0.3.23";
      depKeys = [
        ("futures-core-0.3.23")
        ("futures-io-0.3.23")
        ("futures-task-0.3.23")
        ("memchr-2.5.0")
        ("pin-project-lite-0.2.9")
        ("pin-utils-0.1.0")
        ("slab-0.4.7")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-8IKKVHHjQCKcEcd8qAAXk3zjxYy3iKF+XxwtXEhalXc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/futures-util/0.3.23/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("alloc")
        ("futures-io")
        ("io")
        ("memchr")
        ("slab")
        ("std")
      ];
    };
    "h2-0.3.14" = {
      pname = "h2";
      version = "0.3.14";
      depKeys = [
        ("bytes-1.2.1")
        ("fnv-1.0.7")
        ("futures-core-0.3.23")
        ("futures-sink-0.3.23")
        ("futures-util-0.3.23")
        ("http-0.2.8")
        ("indexmap-1.9.1")
        ("slab-0.4.7")
        ("tokio-1.20.1")
        ("tokio-util-0.7.3")
        ("tracing-0.1.36")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-XKMlks8hrHzKsYJc2H9smz2QIsRNCGFy7QlmvsivML4=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/h2/0.3.14/download";
      });
      edition = "2018";
    };
    "hashbrown-0.12.3" = {
      pname = "hashbrown";
      version = "0.12.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-ip7nDEOq9BfJFDlmRaD6hSYkgBsk67eueP6CcoiayIg=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/hashbrown/0.12.3/download";
      });
      edition = "2021";
      features = [
        ("raw")
      ];
    };
    "hermit-abi-0.1.19" = {
      pname = "hermit-abi";
      version = "0.1.19";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-YrRnNDuUukdtyyUA0kLa27OVV9+IkxCsd8XZkQCqrDM=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.19/download";
      });
      edition = "2018";
      features = [
        ("default")
      ];
    };
    "http-0.2.8" = {
      pname = "http";
      version = "0.2.8";
      depKeys = [
        ("bytes-1.2.1")
        ("fnv-1.0.7")
        ("itoa-1.0.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-dfQ9QeJplcF+ce4SZFHdOUEBCwUUqBqdEfOzQd68I5k=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/http/0.2.8/download";
      });
      edition = "2018";
    };
    "http-body-0.4.5" = {
      pname = "http-body";
      version = "0.4.5";
      depKeys = [
        ("bytes-1.2.1")
        ("http-0.2.8")
        ("pin-project-lite-0.2.9")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-1fOPFtGE428kCKVSgc1ljsvTygXM5tZRChduyjk+JtE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/http-body/0.4.5/download";
      });
      edition = "2018";
    };
    "httparse-1.7.1" = {
      pname = "httparse";
      version = "1.7.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-SWzim7WlJ4W0Tg98ooR64LuDnJvSj2msrJuZ1GHAwEw=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/httparse/1.7.1/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("default")
        ("std")
      ];
    };
    "httpdate-1.0.2" = {
      pname = "httpdate";
      version = "1.0.2";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-xKHjbIIdvgRXT2AoSKGfdC9Ps8mNQESfEbytGNaxdCE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/httpdate/1.0.2/download";
      });
      edition = "2018";
    };
    "humantime-2.1.0" = {
      pname = "humantime";
      version = "2.1.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-mjpb+xlZMe6zNrKntNdh2uyEG5f5R9NDlGAXN6e7peQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/humantime/2.1.0/download";
      });
      edition = "2018";
    };
    "hyper-0.14.20" = {
      pname = "hyper";
      version = "0.14.20";
      depKeys = [
        ("bytes-1.2.1")
        ("futures-channel-0.3.23")
        ("futures-core-0.3.23")
        ("futures-util-0.3.23")
        ("h2-0.3.14")
        ("http-0.2.8")
        ("http-body-0.4.5")
        ("httparse-1.7.1")
        ("httpdate-1.0.2")
        ("itoa-1.0.3")
        ("pin-project-lite-0.2.9")
        ("socket2-0.4.6")
        ("tokio-1.20.1")
        ("tower-service-0.3.2")
        ("tracing-0.1.36")
        ("want-0.3.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-Askp3Fw54zWgPEBSknKBGIYHIbEBkNmMKg8O/Vuq+6w=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/hyper/0.14.20/download";
      });
      edition = "2018";
      features = [
        ("client")
        ("h2")
        ("http1")
        ("http2")
        ("runtime")
        ("socket2")
        ("tcp")
      ];
    };
    "hyper-tls-0.5.0" = {
      pname = "hyper-tls";
      version = "0.5.0";
      depKeys = [
        ("bytes-1.2.1")
        ("hyper-0.14.20")
        ("native-tls-0.2.10")
        ("tokio-1.20.1")
        ("tokio-native-tls-0.3.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-1hg936mbhdphoUC+oO/JP99WzqoEGzfVU1GAMIJ/mQU=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/hyper-tls/0.5.0/download";
      });
      edition = "2018";
    };
    "idna-0.2.3" = {
      pname = "idna";
      version = "0.2.3";
      depKeys = [
        ("matches-0.1.9")
        ("unicode-bidi-0.3.8")
        ("unicode-normalization-0.1.21")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-QYoKb6uCFHX2NO/jzMRcAT90Lv4D2FPo0zVdXLhQ7Pg=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/idna/0.2.3/download";
      });
      edition = "2018";
    };
    "indexmap-1.9.1" = {
      pname = "indexmap";
      version = "1.9.1";
      depKeys = [
        ("hashbrown-0.12.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-EKNal3MDIP/o4tQQtdO2knm5jSwUvbi3Dqiez3iI1B4=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/indexmap/1.9.1/download";
      });
      buildDepKeys = [
        ("autocfg-1.1.0")
      ];
      buildSrc = "build.rs";
      edition = "2021";
      features = [
        ("std")
      ];
    };
    "instant-0.1.12" = {
      pname = "instant";
      version = "0.1.12";
      depKeys = [
        ("cfg-if-1.0.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-elu+gkxQfF2llWNV6Gp0bYLg4UZPZdhizF5x2nDpSyw=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/instant/0.1.12/download";
      });
      edition = "2018";
    };
    "ipnet-2.5.0" = {
      pname = "ipnet";
      version = "2.5.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-h51Ug0yMdkV+9Ck6aJsqjFmwdgZ613sV76+7BfkqWSs=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/ipnet/2.5.0/download";
      });
      edition = "2018";
      features = [
        ("default")
      ];
    };
    "is_executable-1.0.1" = {
      pname = "is_executable";
      version = "1.0.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-+prNxtZ7deYmrWRHNOi8bfiT2c0qg0EpBl091hWOqcg=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/is_executable/1.0.1/download";
      });
      edition = "2015";
    };
    "itertools-0.10.3" = {
      pname = "itertools";
      version = "0.10.3";
      depKeys = [
        ("either-1.8.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-qanRn6Hnm2IV/ym51ogLcGFH8W6bHbseTllHtbArxeM=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/itertools/0.10.3/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("use_alloc")
        ("use_std")
      ];
    };
    "itoa-1.0.3" = {
      pname = "itoa";
      version = "1.0.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-bIr4RnT+HyI6mCyTOg7hCGrE1AUqoPuAYMEsatg451Q=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/itoa/1.0.3/download";
      });
      edition = "2018";
    };
    "js-sys-0.3.59" = {
      pname = "js-sys";
      version = "0.3.59";
      depKeys = [
        ("wasm-bindgen-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-JYRRqxCzT4r1NBbR/atywi6AXwySoRNtWUcOwLEROLI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/js-sys/0.3.59/download";
      });
      edition = "2018";
    };
    "lazy_static-1.4.0" = {
      pname = "lazy_static";
      version = "1.4.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-4qutI/vEKzcA8vJ5hE3IMq2ysusGmy35GPRVxOGMxkY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download";
      });
      edition = "2015";
    };
    "libc-0.2.132" = {
      pname = "libc";
      version = "0.2.132";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-g3Hk5TQcOpbbEn6yRlrGgc7UxDPgHdDpOK2+8mupO6U=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/libc/0.2.132/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("default")
        ("std")
      ];
    };
    "log-0.4.17" = {
      pname = "log";
      version = "0.4.17";
      depKeys = [
        ("cfg-if-1.0.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-q7EuaHz7RKpA9B/Dl473ZEj5tgOMrWrvQlnTwJWiOC4=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/log/0.4.17/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("std")
      ];
    };
    "matches-0.1.9" = {
      pname = "matches";
      version = "0.1.9";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-o+N4tmoGDUiUe1kHN7MKG+dnBsjde4ug8v45icaKhT8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/matches/0.1.9/download";
      });
      edition = "2015";
      libPath = "lib.rs";
    };
    "memchr-2.5.0" = {
      pname = "memchr";
      version = "2.5.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-Lf/lLs8ndy5gGQW3Uiy073kNLMIDSIu9Di/oX8t0Vm0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/memchr/2.5.0/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "memmap2-0.5.7" = {
      pname = "memmap2";
      version = "0.5.7";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-la8V80WxevLvyOrWCA+4vDdvjOwbNSd7k1Y3WV/ndJg=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/memmap2/0.5.7/download";
      });
      edition = "2018";
    };
    "mime-0.3.16" = {
      pname = "mime";
      version = "0.3.16";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-KmDHzlAcceA6nJwNNbhhQTrpJb2XnMek4w0GAGmqrI0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/mime/0.3.16/download";
      });
      edition = "2015";
    };
    "mio-0.8.4" = {
      pname = "mio";
      version = "0.8.4";
      depKeys = [
        ("libc-0.2.132")
        ("log-0.4.17")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-V+4cI8fGOwySUMM5/9xpJV8RCymLkBufbIJUe3uHyq8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/mio/0.8.4/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("net")
        ("os-ext")
        ("os-poll")
      ];
    };
    "native-tls-0.2.10" = {
      pname = "native-tls";
      version = "0.2.10";
      depKeys = [
        ("lazy_static-1.4.0")
        ("libc-0.2.132")
        ("security-framework-2.7.0")
        ("security-framework-sys-2.6.1")
        ("tempfile-3.3.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-/X4vNhhVf5gOCxfohWJS7uPJf6EsVN/wyikPtiZspKk=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/native-tls/0.2.10/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
    };
    "nix-nar-0.2.0" = {
      pname = "nix-nar";
      version = "0.2.0";
      depKeys = [
        ("is_executable-1.0.1")
        ("thiserror-1.0.34")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-25UO47TGhfYh6T6gdDX+WpVoJOP/WfRb3HKllLk0P5w=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/nix-nar/0.2.0/download";
      });
      edition = "2021";
    };
    "num_cpus-1.13.1" = {
      pname = "num_cpus";
      version = "1.13.1";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-GeZFJuve4YI0FXLlDprQOWWqUQzZRCekVJRI8oXpV6E=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/num_cpus/1.13.1/download";
      });
      edition = "2015";
    };
    "once_cell-1.13.1" = {
      pname = "once_cell";
      version = "1.13.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-B0hk2iBrSXO4TrkWgwINvv1qjD8POOBU2TlU6JGTXk4=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/once_cell/1.13.1/download";
      });
      edition = "2018";
      features = [
        ("alloc")
        ("default")
        ("race")
        ("std")
      ];
    };
    "openssl-0.10.41" = {
      pname = "openssl";
      version = "0.10.41";
      depKeys = [
        ("bitflags-1.3.2")
        ("cfg-if-1.0.0")
        ("foreign-types-0.3.2")
        ("libc-0.2.132")
        ("once_cell-1.13.1")
        ("openssl-macros-0.1.0")
        ("openssl-sys-0.9.75")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-YY/r9lM2SQ388gtz+IX1ZRoMicZMLUqMNmJYWnC/W9A=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/openssl/0.10.41/download";
      });
      buildSrc = "build.rs";
      crateRenames = {
        openssl-sys = [
          ({
            rename = "ffi";
            version = "0.9.75";
          })
        ];
      };
      edition = "2018";
    };
    "openssl-macros-0.1.0" = {
      pname = "openssl-macros";
      version = "0.1.0";
      depKeys = [
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("syn-1.0.99")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-tQHkTxFmWWDH5/zwYsfZahSt5KqYEWwASy43tb59c2w=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/openssl-macros/0.1.0/download";
      });
      edition = "2018";
      procMacro = true;
    };
    "openssl-probe-0.1.5" = {
      pname = "openssl-probe";
      version = "0.1.5";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-/wEaMCw5alGXaSQx/BlIAZFUr8F4uvfY43NnRCpGAc8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/openssl-probe/0.1.5/download";
      });
      edition = "2015";
    };
    "openssl-sys-0.9.75" = {
      pname = "openssl-sys";
      version = "0.9.75";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-5fm9DCcQVBo82nPW+axPGyQN5K4mEGXTCdvnPZ3OtC8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/openssl-sys/0.9.75/download";
      });
      buildDepKeys = [
        ("autocfg-1.1.0")
        ("cc-1.0.73")
        ("pkg-config-0.3.25")
      ];
      buildSrc = "build/main.rs";
      edition = "2015";
    };
    "percent-encoding-2.1.0" = {
      pname = "percent-encoding";
      version = "2.1.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-1P1WQdAcjxiiPae2/ikpj/S1WvzM33iXOyTPMXX+4y4=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/percent-encoding/2.1.0/download";
      });
      edition = "2015";
      libPath = "lib.rs";
    };
    "pin-project-lite-0.2.9" = {
      pname = "pin-project-lite";
      version = "0.2.9";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-4KeuOsLxFzCF05hTHHBXVslKTFaEN4XfhaYMGgr6wRY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/pin-project-lite/0.2.9/download";
      });
      edition = "2018";
    };
    "pin-utils-0.1.0" = {
      pname = "pin-utils";
      version = "0.1.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-i4cNjBUbby+5PoShMUYTjwXQLtEcfnxU+IJqqvfJ8YQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/pin-utils/0.1.0/download";
      });
      edition = "2018";
    };
    "pkg-config-0.3.25" = {
      pname = "pkg-config";
      version = "0.3.25";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-HfjE7EsGJ+U73yFGFa0oc2fkglWM+EsQklCzdGTcA64=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.25/download";
      });
      edition = "2015";
    };
    "proc-macro2-1.0.43" = {
      pname = "proc-macro2";
      version = "1.0.43";
      depKeys = [
        ("unicode-ident-1.0.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-CiyixhvJ89dNKIYpSre5hTq9nBrZA6OseBXFiYm7e6s=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.43/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("proc-macro")
      ];
    };
    "quote-1.0.21" = {
      pname = "quote";
      version = "1.0.21";
      depKeys = [
        ("proc-macro2-1.0.43")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-u+RI83en1pYeMPWVX5uNEGw/XkSdST7hsSXB1DwrUXk=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/quote/1.0.21/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("proc-macro")
      ];
    };
    "redox_syscall-0.2.16" = {
      pname = "redox_syscall";
      version = "0.2.16";
      depKeys = [
        ("bitflags-1.3.2")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-+1pYwYVbS2gZ1ZASFVYD8LIq0wytdSYAqt/LaVJlUZo=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/redox_syscall/0.2.16/download";
      });
      edition = "2018";
    };
    "regex-1.6.0" = {
      pname = "regex";
      version = "1.6.0";
      depKeys = [
        ("aho-corasick-0.7.18")
        ("memchr-2.5.0")
        ("regex-syntax-0.6.27")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-TE6zJnF0uMbC9lQRZiORCg/vCcR1P43YPbKcSKDfmIs=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/regex/1.6.0/download";
      });
      edition = "2018";
      features = [
        ("aho-corasick")
        ("memchr")
        ("perf")
        ("perf-cache")
        ("perf-dfa")
        ("perf-inline")
        ("perf-literal")
        ("std")
      ];
    };
    "regex-syntax-0.6.27" = {
      pname = "regex-syntax";
      version = "0.6.27";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-o/h7c84RsWGaPGMy9FNB4ARxc3cei4tz+Hv+77e1YkQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/regex-syntax/0.6.27/download";
      });
      edition = "2018";
    };
    "remove_dir_all-0.5.3" = {
      pname = "remove_dir_all";
      version = "0.5.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-Os0SVmVCKXOjOsnT3S34XtrQ9K6bANr7GgXkOp9e+Oc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/remove_dir_all/0.5.3/download";
      });
      edition = "2015";
    };
    "reqwest-0.11.11" = {
      pname = "reqwest";
      version = "0.11.11";
      depKeys = [
        ("base64-0.13.0")
        ("bytes-1.2.1")
        ("encoding_rs-0.8.31")
        ("futures-core-0.3.23")
        ("futures-util-0.3.23")
        ("h2-0.3.14")
        ("http-0.2.8")
        ("http-body-0.4.5")
        ("hyper-0.14.20")
        ("hyper-tls-0.5.0")
        ("ipnet-2.5.0")
        ("lazy_static-1.4.0")
        ("log-0.4.17")
        ("mime-0.3.16")
        ("native-tls-0.2.10")
        ("percent-encoding-2.1.0")
        ("pin-project-lite-0.2.9")
        ("serde-1.0.144")
        ("serde_urlencoded-0.7.1")
        ("tokio-1.20.1")
        ("tokio-native-tls-0.3.0")
        ("tower-service-0.3.2")
        ("url-2.2.2")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-t1qmmj8Gu8xm7eM68q8lPG96hrHKADP2DFgKJwdPv5I=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/reqwest/0.11.11/download";
      });
      crateRenames = {
        native-tls = [
          ({
            rename = "native_tls_crate";
            version = "0.2.10";
          })
        ];
      };
      edition = "2018";
      features = [
        ("__tls")
        ("blocking")
        ("default")
        ("default-tls")
        ("hyper-tls")
        ("native-tls-crate")
        ("tokio-native-tls")
      ];
    };
    "runix-0.1.0" = {
      pname = "runix";
      version = "0.1.0";
      depKeys = [
        ("anyhow-1.0.62")
        ("env_logger-0.9.0")
        ("itertools-0.10.3")
        ("log-0.4.17")
        ("memmap2-0.5.7")
        ("nix-nar-0.2.0")
        ("reqwest-0.11.11")
        ("serde-1.0.144")
        ("serde_json-1.0.85")
        ("walkdir-2.3.2")
      ];
      crateBin = [
        ({
          name = "runix";
          path = "src/main.rs";
        })
      ];
      edition = "2021";
    };
    "ryu-1.0.11" = {
      pname = "ryu";
      version = "1.0.11";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-RQGr3/OugqHBtHehclLrac7p5m65FcGrqk9E2HPfnwk=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/ryu/1.0.11/download";
      });
      edition = "2018";
    };
    "same-file-1.0.6" = {
      pname = "same-file";
      version = "1.0.6";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-k/wdw6qpv+2V4C5urau0uvfjB4sL0bTXtrC2g3iQBQI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/same-file/1.0.6/download";
      });
      edition = "2018";
    };
    "schannel-0.1.20" = {
      pname = "schannel";
      version = "0.1.20";
      depKeys = [
        ("lazy_static-1.4.0")
        ("windows-sys-0.36.1")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-iNZzEUZGLqJdkkSy7V/R1xbSXFLk1Uqk+w88TphU2+I=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/schannel/0.1.20/download";
      });
      edition = "2018";
    };
    "security-framework-2.7.0" = {
      pname = "security-framework";
      version = "2.7.0";
      depKeys = [
        ("bitflags-1.3.2")
        ("core-foundation-0.9.3")
        ("core-foundation-sys-0.8.3")
        ("libc-0.2.132")
        ("security-framework-sys-2.6.1")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-K8G7l4BK9mMYE8VXOfdxBx4PLtM+4gtoyG7FBdkGNWw=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/security-framework/2.7.0/download";
      });
      edition = "2021";
      features = [
        ("OSX_10_9")
        ("default")
      ];
    };
    "security-framework-sys-2.6.1" = {
      pname = "security-framework-sys";
      version = "2.6.1";
      depKeys = [
        ("core-foundation-sys-0.8.3")
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-AWChOhd6Rb+0POccAVgJmEdPVWrYVNy8qTbdKEGlxVY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/security-framework-sys/2.6.1/download";
      });
      edition = "2018";
      features = [
        ("OSX_10_9")
        ("default")
      ];
    };
    "serde-1.0.144" = {
      pname = "serde";
      version = "1.0.144";
      depKeys = [
        ("serde_derive-1.0.144")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-D3R3EN49zUO4jJFodzJU6AnY3b35ZTuE4lVKshnxeGA=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/serde/1.0.144/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("default")
        ("derive")
        ("serde_derive")
        ("std")
      ];
    };
    "serde_derive-1.0.144" = {
      pname = "serde_derive";
      version = "1.0.144";
      depKeys = [
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("syn-1.0.99")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-lO06gW+x0QGBL4PnifiIMiw04pH4lPGVkNwxCWPoegA=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/serde_derive/1.0.144/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("default")
      ];
      procMacro = true;
    };
    "serde_json-1.0.85" = {
      pname = "serde_json";
      version = "1.0.85";
      depKeys = [
        ("itoa-1.0.3")
        ("ryu-1.0.11")
        ("serde-1.0.144")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-5Voo46rvnVzgUG0KFNu6gFTdx+SZ71It2LJoWeydSkQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/serde_json/1.0.85/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "serde_urlencoded-0.7.1" = {
      pname = "serde_urlencoded";
      version = "0.7.1";
      depKeys = [
        ("form_urlencoded-1.0.1")
        ("itoa-1.0.3")
        ("ryu-1.0.11")
        ("serde-1.0.144")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-00kcFHFcoilMTWqI8V6Ec5eIwdAw7tjBEENqr9qi8/0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/serde_urlencoded/0.7.1/download";
      });
      edition = "2018";
    };
    "slab-0.4.7" = {
      pname = "slab";
      version = "0.4.7";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-RhSnayqL4AWMqp27r2bZiFJ9htADwRqU+9M112Ye3O8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/slab/0.4.7/download";
      });
      buildDepKeys = [
        ("autocfg-1.1.0")
      ];
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "socket2-0.4.6" = {
      pname = "socket2";
      version = "0.4.6";
      depKeys = [
        ("libc-0.2.132")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-EMmLujcbmyKnGpQU5CD5Ld6yNpI5rwggCBYWnV4t16o=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/socket2/0.4.6/download";
      });
      edition = "2018";
      features = [
        ("all")
      ];
    };
    "syn-1.0.99" = {
      pname = "syn";
      version = "1.0.99";
      depKeys = [
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("unicode-ident-1.0.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-WNvvbsZVBV4guGsVqMxtQ5zKGbZnU3rGoTaVctFRqxM=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/syn/1.0.99/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("clone-impls")
        ("default")
        ("derive")
        ("full")
        ("parsing")
        ("printing")
        ("proc-macro")
        ("quote")
        ("visit")
      ];
    };
    "tempfile-3.3.0" = {
      pname = "tempfile";
      version = "3.3.0";
      depKeys = [
        ("cfg-if-1.0.0")
        ("fastrand-1.8.0")
        ("libc-0.2.132")
        ("remove_dir_all-0.5.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-XNse9OrurdyPvTceUBcFcGSvCRGQLvNrOYAfZ8xteeQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tempfile/3.3.0/download";
      });
      crateRenames = {
        redox_syscall = [
          ({
            rename = "syscall";
            version = "0.2.16";
          })
        ];
      };
      edition = "2018";
    };
    "termcolor-1.1.3" = {
      pname = "termcolor";
      version = "1.1.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-urJNMLkRsjdvOhPMLNRDFC8Mgd2gTBGGk+NbODV1d1U=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/termcolor/1.1.3/download";
      });
      edition = "2018";
    };
    "thiserror-1.0.34" = {
      pname = "thiserror";
      version = "1.0.34";
      depKeys = [
        ("thiserror-impl-1.0.34")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-jBsFyp0Qa6fS4xqdq0pk574szkFTIZZuoxMsSaZW4lI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/thiserror/1.0.34/download";
      });
      edition = "2018";
    };
    "thiserror-impl-1.0.34" = {
      pname = "thiserror-impl";
      version = "1.0.34";
      depKeys = [
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("syn-1.0.99")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-6PJZGYNkLehckhAV8/BwxmWhl+1p5BevQ2EV46FAdIc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/thiserror-impl/1.0.34/download";
      });
      edition = "2018";
      procMacro = true;
    };
    "tinyvec-1.6.0" = {
      pname = "tinyvec";
      version = "1.6.0";
      depKeys = [
        ("tinyvec_macros-0.1.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-h8xc6zh1uyDCiQAFpOImpGUSZKXHXtskIbUoYaCgy1A=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tinyvec/1.6.0/download";
      });
      edition = "2018";
      features = [
        ("alloc")
        ("default")
        ("tinyvec_macros")
      ];
    };
    "tinyvec_macros-0.1.0" = {
      pname = "tinyvec_macros";
      version = "0.1.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-zadNp+GmZPeVux+Kh+xAb7iaAlIs9uUGINAWrdbbv1w=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tinyvec_macros/0.1.0/download";
      });
      edition = "2018";
    };
    "tokio-1.20.1" = {
      pname = "tokio";
      version = "1.20.1";
      depKeys = [
        ("bytes-1.2.1")
        ("libc-0.2.132")
        ("memchr-2.5.0")
        ("mio-0.8.4")
        ("num_cpus-1.13.1")
        ("once_cell-1.13.1")
        ("pin-project-lite-0.2.9")
        ("socket2-0.4.6")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-eoMl9jp9R3TdBB42OyQJ7Rxcu9D4Z3leZh3wZrKwpYE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tokio/1.20.1/download";
      });
      buildDepKeys = [
        ("autocfg-1.1.0")
      ];
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("bytes")
        ("default")
        ("io-util")
        ("libc")
        ("memchr")
        ("mio")
        ("net")
        ("num_cpus")
        ("once_cell")
        ("rt")
        ("rt-multi-thread")
        ("socket2")
        ("sync")
        ("time")
        ("winapi")
      ];
    };
    "tokio-native-tls-0.3.0" = {
      pname = "tokio-native-tls";
      version = "0.3.0";
      depKeys = [
        ("native-tls-0.2.10")
        ("tokio-1.20.1")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-99mVZgvSt/jBVoQUwRJgdsE/u3JcQBEtwBILeOubcXs=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tokio-native-tls/0.3.0/download";
      });
      edition = "2018";
    };
    "tokio-util-0.7.3" = {
      pname = "tokio-util";
      version = "0.7.3";
      depKeys = [
        ("bytes-1.2.1")
        ("futures-core-0.3.23")
        ("futures-sink-0.3.23")
        ("pin-project-lite-0.2.9")
        ("tokio-1.20.1")
        ("tracing-0.1.36")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-zEY82N7dw3cNIPmFIUPVC/YJTmQLSFyy4YmiCZCF/0U=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tokio-util/0.7.3/download";
      });
      edition = "2018";
      features = [
        ("codec")
        ("default")
        ("tracing")
      ];
    };
    "tower-service-0.3.2" = {
      pname = "tower-service";
      version = "0.3.2";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-trwcnOK1E1rH+TxykY/Df+uHK9xqVTOouF60uGv9rlI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tower-service/0.3.2/download";
      });
      edition = "2018";
    };
    "tracing-0.1.36" = {
      pname = "tracing";
      version = "0.1.36";
      depKeys = [
        ("cfg-if-1.0.0")
        ("pin-project-lite-0.2.9")
        ("tracing-core-0.1.29")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-L86VZ71gpn0IoWSIdWchujkvJPKQBkAogeQ7GarGQwc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tracing/0.1.36/download";
      });
      edition = "2018";
      features = [
        ("std")
      ];
    };
    "tracing-core-0.1.29" = {
      pname = "tracing-core";
      version = "0.1.29";
      depKeys = [
        ("once_cell-1.13.1")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-Wu6kMDB2VYoAcUuCP5rWfViju9od+D2IJ9IRkxVuIvc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/tracing-core/0.1.29/download";
      });
      edition = "2018";
      features = [
        ("once_cell")
        ("std")
      ];
    };
    "try-lock-0.2.3" = {
      pname = "try-lock";
      version = "0.2.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-WVR7znHZw4uD2cDpK2BmxCUzcfFQBd7www2WV/UMdkI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/try-lock/0.2.3/download";
      });
      edition = "2015";
    };
    "unicode-bidi-0.3.8" = {
      pname = "unicode-bidi";
      version = "0.3.8";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-CZtxKDAdKF953dVbmoPV5rnpfJLg6g2uvucmPpMt6ZI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/unicode-bidi/0.3.8/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("hardcoded-data")
        ("std")
      ];
    };
    "unicode-ident-1.0.3" = {
      pname = "unicode-ident";
      version = "1.0.3";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-xPWzehVJmajz+YzCOmKNhQ4VRHnNlN7PNBRpbhLjGq8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/unicode-ident/1.0.3/download";
      });
      edition = "2018";
    };
    "unicode-normalization-0.1.21" = {
      pname = "unicode-normalization";
      version = "0.1.21";
      depKeys = [
        ("tinyvec-1.6.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-hUy9xPe8auGcgg1Eq9wyd6w+GyuT2yCmNoJdkyL7YOY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/unicode-normalization/0.1.21/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "url-2.2.2" = {
      pname = "url";
      version = "2.2.2";
      depKeys = [
        ("form_urlencoded-1.0.1")
        ("idna-0.2.3")
        ("matches-0.1.9")
        ("percent-encoding-2.1.0")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-pQfDg7LTO1/DXRhh535rOD0Viy2l4U/lG4Pf7fb9V4w=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/url/2.2.2/download";
      });
      edition = "2018";
    };
    "vcpkg-0.2.15" = {
      pname = "vcpkg";
      version = "0.2.15";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-rM1Opi97t6gv4jBm+wlX1I72d/buuCFfNy9S5IuzJCY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/vcpkg/0.2.15/download";
      });
      edition = "2015";
    };
    "walkdir-2.3.2" = {
      pname = "walkdir";
      version = "2.3.2";
      depKeys = [
        ("same-file-1.0.6")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-gIzyc1zUtoZhE/ZIt5HGrcVxRTe8Ii2TR7sgM4b/2lY=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/walkdir/2.3.2/download";
      });
      edition = "2018";
    };
    "want-0.3.0" = {
      pname = "want";
      version = "0.3.0";
      depKeys = [
        ("log-0.4.17")
        ("try-lock-0.2.3")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-HOipaMsc0RDRNv+LgZpVbW+22Rk2PGFTT2hgx+sXK6A=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/want/0.3.0/download";
      });
      edition = "2018";
    };
    "wasi-0.11.0+wasi-snapshot-preview1" = {
      pname = "wasi";
      version = "0.11.0-wasi-snapshot-preview1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-nI2H5ytko7TbKNEc4pI3wkYYj09RBX1lp+q2O3mH5CM=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasi/0.11.0+wasi-snapshot-preview1/download";
      });
      edition = "2018";
      features = [
        ("default")
        ("std")
      ];
    };
    "wasm-bindgen-0.2.82" = {
      pname = "wasm-bindgen";
      version = "0.2.82";
      depKeys = [
        ("cfg-if-1.0.0")
        ("wasm-bindgen-macro-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-/HZS4/bEcGyNnNVIMsSkzLm1M24sO9FU1czPvxwfX30=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.82/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
      features = [
        ("default")
        ("spans")
        ("std")
      ];
    };
    "wasm-bindgen-backend-0.2.82" = {
      pname = "wasm-bindgen-backend";
      version = "0.2.82";
      depKeys = [
        ("bumpalo-3.11.0")
        ("log-0.4.17")
        ("once_cell-1.13.1")
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("syn-1.0.99")
        ("wasm-bindgen-shared-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-ZizUSAVYa9UpcblYax34XNu9kRLk702PQVWcM03GrD8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen-backend/0.2.82/download";
      });
      edition = "2018";
      features = [
        ("spans")
      ];
    };
    "wasm-bindgen-futures-0.4.32" = {
      pname = "wasm-bindgen-futures";
      version = "0.4.32";
      depKeys = [
        ("cfg-if-1.0.0")
        ("js-sys-0.3.59")
        ("wasm-bindgen-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-+nb7Ihofis3fW1Ss6FkSYGmArWYax6UDtFcP/TpiTa0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen-futures/0.4.32/download";
      });
      edition = "2018";
    };
    "wasm-bindgen-macro-0.2.82" = {
      pname = "wasm-bindgen-macro";
      version = "0.2.82";
      depKeys = [
        ("quote-1.0.21")
        ("wasm-bindgen-macro-support-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-smDxPTASBx37FRKEnAM7GSUDg3OupIztMBLAnflSxgI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.82/download";
      });
      edition = "2018";
      features = [
        ("spans")
      ];
      procMacro = true;
    };
    "wasm-bindgen-macro-support-0.2.82" = {
      pname = "wasm-bindgen-macro-support";
      version = "0.2.82";
      depKeys = [
        ("proc-macro2-1.0.43")
        ("quote-1.0.21")
        ("syn-1.0.99")
        ("wasm-bindgen-backend-0.2.82")
        ("wasm-bindgen-shared-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-W+jmVL3Zt5IWwpKauQchqoL69lxIzfCL3E5/UTV7gNo=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.82/download";
      });
      edition = "2018";
      features = [
        ("spans")
      ];
    };
    "wasm-bindgen-shared-0.2.82" = {
      pname = "wasm-bindgen-shared";
      version = "0.2.82";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-ZZjdC9PH1RCV/2UxpbI+AqzcgYBOMNjwevt3tyFaFAo=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.82/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "web-sys-0.3.59" = {
      pname = "web-sys";
      version = "0.3.59";
      depKeys = [
        ("js-sys-0.3.59")
        ("wasm-bindgen-0.2.82")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-7QVasn+UFCMZfrhrIDVyCxo85AUE3wgsrC7MbtczNaE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/web-sys/0.3.59/download";
      });
      edition = "2018";
      features = [
        ("Blob")
        ("BlobPropertyBag")
        ("Event")
        ("EventTarget")
        ("File")
        ("FormData")
        ("Headers")
        ("MessageEvent")
        ("Request")
        ("RequestCredentials")
        ("RequestInit")
        ("RequestMode")
        ("Response")
        ("ServiceWorkerGlobalScope")
        ("Window")
        ("Worker")
        ("WorkerGlobalScope")
      ];
    };
    "winapi-0.3.9" = {
      pname = "winapi";
      version = "0.3.9";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-XIOaZ0/NepiVLlkyQupACr6TmSdGdh44ZBQF0osA9Bk=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
      features = [
        ("consoleapi")
        ("errhandlingapi")
        ("fileapi")
        ("handleapi")
        ("impl-debug")
        ("impl-default")
        ("minwinbase")
        ("minwindef")
        ("namedpipeapi")
        ("processenv")
        ("std")
        ("timezoneapi")
        ("winbase")
        ("wincon")
        ("winerror")
        ("winnt")
        ("winreg")
        ("ws2ipdef")
        ("ws2tcpip")
      ];
    };
    "winapi-i686-pc-windows-gnu-0.4.0" = {
      pname = "winapi-i686-pc-windows-gnu";
      version = "0.4.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-rDuHxjYgQm3ZuZHlzgMp7/VFvMu7NPO+Cf9vtqtRt7Y=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
    };
    "winapi-util-0.1.5" = {
      pname = "winapi-util";
      version = "0.1.5";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-cOxs6FuxWBUcrl5ch/lajpfSwMSwASI/M6M0485d4Xg=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/winapi-util/0.1.5/download";
      });
      edition = "2018";
    };
    "winapi-x86_64-pc-windows-gnu-0.4.0" = {
      pname = "winapi-x86_64-pc-windows-gnu";
      version = "0.4.0";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-cS4ieEHQV8HuHNL7Ivp+WlRhro5I+iynnsQs/BkxGD8=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
    };
    "windows-sys-0.36.1" = {
      pname = "windows-sys";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-6gQVWhaln56reG/hKkpFDnXNsXX54NgNoeF9sJ9VuNI=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows-sys/0.36.1/download";
      });
      edition = "2018";
      features = [
        ("Win32")
        ("Win32_Foundation")
        ("Win32_Networking")
        ("Win32_Networking_WinSock")
        ("Win32_Security")
        ("Win32_Security_Authentication")
        ("Win32_Security_Authentication_Identity")
        ("Win32_Security_Credentials")
        ("Win32_Security_Cryptography")
        ("Win32_Storage")
        ("Win32_Storage_FileSystem")
        ("Win32_System")
        ("Win32_System_IO")
        ("Win32_System_Memory")
        ("Win32_System_Pipes")
        ("Win32_System_WindowsProgramming")
        ("default")
      ];
    };
    "windows_aarch64_msvc-0.36.1" = {
      pname = "windows_aarch64_msvc";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-m7jD/Tmt4tZ+mHSsTz2yHw1xC+4A/nyrFpSewYTuqkc=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows_aarch64_msvc/0.36.1/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "windows_i686_gnu-0.36.1" = {
      pname = "windows_i686_gnu";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-GA5szwHa9MQmuEbfxm2x/FGPB0uqeTqn2bmq7/rWo7Y=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows_i686_gnu/0.36.1/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "windows_i686_msvc-0.36.1" = {
      pname = "windows_i686_msvc";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-4ueRcUiygS0e6vrrIql+SBPfpgo/j3jr4gS8yI8S8CQ=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows_i686_msvc/0.36.1/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "windows_x86_64_gnu-0.36.1" = {
      pname = "windows_x86_64_gnu";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-Tc0XG4d2xBuXUh5doSei2GrSgBFIB9Cyqx5GK8dk2eE=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows_x86_64_gnu/0.36.1/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "windows_x86_64_msvc-0.36.1" = {
      pname = "windows_x86_64_msvc";
      version = "0.36.1";
      depKeys = [
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-yBHKSoyFPvQgq9hZK6U927rJBBD6tpA7PnmXKmMfdoA=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/windows_x86_64_msvc/0.36.1/download";
      });
      buildSrc = "build.rs";
      edition = "2018";
    };
    "winreg-0.10.1" = {
      pname = "winreg";
      version = "0.10.1";
      depKeys = [
        ("winapi-0.3.9")
      ];
      src = (pkgs.fetchurl {
        hash = "sha256-gND04nLIXe8TlHY4CxL5rGCSZondLgHUkjIi9AWAhp0=";
        name = "crate.tar.gz";
        url = "https://crates.io/api/v1/crates/winreg/0.10.1/download";
      });
      buildSrc = "build.rs";
      edition = "2015";
    };
  };
}