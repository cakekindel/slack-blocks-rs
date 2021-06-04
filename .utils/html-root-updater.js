const {pick} = require('./common');

/// # Public
module.exports = {
    readVersion: (srcLibContents) => getVer(srcLibContents),
    writeVersion: (srcLibContents, version) => setVer(srcLibContents, version),
};

const htmlRootUrlPat = /#\!\[doc\(html_root_url = "https:\/\/docs\.rs\/slack-blocks\/([\d\.]+)"\)\]/i;
const getVer = (lib, ver) => pick(htmlRootUrlPat.exec(lib), 1);
const setVer = (lib, ver) => lib.replace( htmlRootUrlPat
                                        , `#![doc(html_root_url = "https://docs.rs/slack-blocks/${ver}")]`
                                        );

test();

function test() {
  const oldVerExpected = "1.2.3";
  const input = { contents: `#![doc(html_root_url = "https://docs.rs/slack-blocks/${oldVerExpected}")]`
                , version: "1.2.3"
                };

  const oldVer = getVer(input.contents);
  if (oldVer !== oldVerExpected) {
    throw new Error(`in src/lib.rs; expected ${oldVerExpected} got ${oldVer}`);
  }

  const newVerExpected = '#![doc(html_root_url = "https://docs.rs/slack-blocks/1.2.3")]';
  const newVer = setVer(input.contents, input.version);
  if (newVer !== newVerExpected) {
    throw new Error(`in src/lib.rs; expected ${newVerExpected} got ${newVer}`);
  }
};
