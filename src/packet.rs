// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// design: minimum packet length is signature + flags + packetlength + CRC32: 
// 1 + 2 + 1 + 4. This is enough to always read the packet length:
// 1 + 2 + 4 + NNNN + CRC32
//          ^
// This lets us have the following steps:
// until start is 0xb3 ; pass through
// until len >1; buffer
// if start != 0xb32 ; passthrough
// until len >6; buffer
// parse version and length || pass through and reset
// until len >= length; buffer
// packet fully usable.

// we need the following state to implement this:
// a buffer (could be a vec<u8> or a more complex thing to achieve zero-copy)
// a call to supply bytes to the core
// a return mechanism to consume those bytes and yield them to user

// parsing / serialisation should be separate to stream management
// 
// Packet - parse / 

#[test]
fn test_write_full_test_event_with_file_content() {}
