use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Address(Address<NetworkUnchecked>),
  Hash([u8; 32]),
  InscriptionId(InscriptionId),
  Integer(u128),
  OutPoint(OutPoint),
  Rune(SpacedRune),
  Sat(Sat),
  SatPoint(SatPoint),
}

impl FromStr for Object {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    use Representation::*;

    match Representation::from_str(s)? {
      Address => Ok(Self::Address(s.parse()?)),
      Decimal | Degree | Percentile | Name => Ok(Self::Sat(s.parse()?)),
      Hash => Ok(Self::Hash(
        peercoin::hashes::sha256::Hash::from_str(s)?.to_byte_array(),
      )),
      InscriptionId => Ok(Self::InscriptionId(s.parse()?)),
      Integer => Ok(Self::Integer(s.parse()?)),
      OutPoint => Ok(Self::OutPoint(s.parse()?)),
      Rune => Ok(Self::Rune(s.parse()?)),
      SatPoint => Ok(Self::SatPoint(s.parse()?)),
    }
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Address(address) => write!(f, "{}", address.clone().assume_checked()),
      Self::Hash(hash) => {
        for byte in hash {
          write!(f, "{byte:02x}")?;
        }
        Ok(())
      }
      Self::InscriptionId(inscription_id) => write!(f, "{inscription_id}"),
      Self::Integer(integer) => write!(f, "{integer}"),
      Self::OutPoint(outpoint) => write!(f, "{outpoint}"),
      Self::Rune(rune) => write!(f, "{rune}"),
      Self::Sat(sat) => write!(f, "{sat}"),
      Self::SatPoint(satpoint) => write!(f, "{satpoint}"),
    }
  }
}

impl Serialize for Object {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for Object {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    DeserializeFromStr::with(deserializer)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, expected: Object) {
      let actual = s.parse::<Object>().unwrap();
      assert_eq!(actual, expected);
      let round_trip = actual.to_string().parse::<Object>().unwrap();
      assert_eq!(round_trip, expected);
    }

    assert_eq!(
      "nvtdijuwxlp".parse::<Object>().unwrap(),
      Object::Sat(Sat(0))
    );
    assert_eq!("a".parse::<Object>().unwrap(), Object::Sat(Sat::LAST));
    assert_eq!(
      "1.1".parse::<Object>().unwrap(),
      Object::Sat(Sat(50 * COIN_VALUE + 1))
    );
    assert_eq!(
      "1°0′0″0‴".parse::<Object>().unwrap(),
      Object::Sat(Sat(2067187500000000))
    );
    assert_eq!("0%".parse::<Object>().unwrap(), Object::Sat(Sat(0)));

    case("0", Object::Integer(0));

    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1",
      Object::InscriptionId(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1"
          .parse()
          .unwrap(),
      ),
    );

    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      Object::Hash([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xcd, 0xef,
      ]),
    );
    case(
      "tpc1q2jxppf7n6rwkyxmhyma0l3ch70u58vda2t3ngn",
      Object::Address(
        "tpc1q2jxppf7n6rwkyxmhyma0l3ch70u58vda2t3ngn"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "TPC1Q2JXPPF7N6RWKYXMHYMA0L3CH70U58VDA2T3NGN",
      Object::Address(
        "TPC1Q2JXPPF7N6RWKYXMHYMA0L3CH70U58VDA2T3NGN"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "tpc1pwcc0vxwl9y2u7t4xkgpwvtsmhckcunntnrzuv35wr6mfv6d5heast7ch7g",
      Object::Address(
        "tpc1pwcc0vxwl9y2u7t4xkgpwvtsmhckcunntnrzuv35wr6mfv6d5heast7ch7g"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "TPC1PWCC0VXWL9Y2U7T4XKGPWVTSMHCKCUNNTNRZUV35WR6MFV6D5HEAST7CH7G",
      Object::Address(
        "TPC1PWCC0VXWL9Y2U7T4XKGPWVTSMHCKCUNNTNRZUV35WR6MFV6D5HEAST7CH7G"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123",
      Object::OutPoint(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123",
      Object::OutPoint(
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456",
      Object::SatPoint(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456",
      Object::SatPoint(
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456"
          .parse()
          .unwrap(),
      ),
    );
    case(
      "A",
      Object::Rune(SpacedRune {
        rune: Rune(0),
        spacers: 0,
      }),
    );
    case(
      "A•A",
      Object::Rune(SpacedRune {
        rune: Rune(26),
        spacers: 1,
      }),
    );
  }
}
