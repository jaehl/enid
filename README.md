# `enid`

[![Latest Version](https://img.shields.io/crates/v/enid)](https://crates.io/crates/enid)
[![Documentation](https://img.shields.io/docsrs/enid)](https://docs.rs/enid/latest/enid/)

An **ENID** (Encrypted Numeric Identifier) is a 40- or 80-bit value, which can
be used as a unique identifier.

An example of a 40-bit ENID:

```text
m6sc7n75
```

And an 80-bit ENID:

```text
y3gx5gxm-mpb8ey39
```

ENIDs are generated by encrypting plaintext bytes so that they appear
pseudo-random. The encrypted bytes are then formatted as a variant of Base32
(Crockford's Base32) that excludes the letters `i`, `l`, `o`, and `u`. Each
group of 40 bits is represented by 8 characters and separated by a hyphen.

This crate does not yet include a method for generating ENIDs, which will be
added in a future version.

Some features of ENIDs:

* Short - ENIDs are 8 or 17 characters long, compared with 36-character UUIDs.
* Uniformly distributed - sequentially-generated ENIDs are unlikely to appear
  similar.
* URL-safe - ENIDs can be used in URLs without percent-encoding.
