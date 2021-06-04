/// # Public
module.exports = {
    writeVersion: (srcLibContents, version) => setVer(srcLibContents, version),
};

const htmlRootUrlPat = /#\!\[doc\(html_root_url = "https:\/\/docs\.rs\/slack-blocks\/[\d\.]+"\)\]/i;
const setVer = (lib, ver) => lib.replace( htmlRootUrlPat
                                        , `#![doc(html_root_url = "https://docs.rs/slack-blocks/${ver}")]`
                                        );

test();

function test() {
  const input = { contents: '#![doc(html_root_url = "https://docs.rs/slack-blocks/0.0.0")]'
                , version: "1.2.3"
                };
  
  const expected = '#![doc(html_root_url = "https://docs.rs/slack-blocks/1.2.3")]';
  const actual = setVer(input.contents, input.version);
  if (actual !== expected) {
      throw new Error(`in src/lib.rs; expected ${expected} got ${actual}`);
  }
};
