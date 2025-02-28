import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XGraphemeClusterBreakIteratorLatin1 } from "./ICU4XGraphemeClusterBreakIteratorLatin1";
import { ICU4XGraphemeClusterBreakIteratorUtf16 } from "./ICU4XGraphemeClusterBreakIteratorUtf16";
import { ICU4XGraphemeClusterBreakIteratorUtf8 } from "./ICU4XGraphemeClusterBreakIteratorUtf8";

/**

 * An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints in strings.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.GraphemeClusterBreakSegmenter.html Rust documentation} for more information.
 */
export class ICU4XGraphemeClusterBreakSegmenter {

  /**

   * Construct an {@link ICU4XGraphemeClusterBreakSegmenter `ICU4XGraphemeClusterBreakSegmenter`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.GraphemeClusterBreakSegmenter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XGraphemeClusterBreakSegmenter | never;

  /**

   * Segments a UTF-8 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_str Rust documentation} for more information.
   */
  segment_utf8(input: string): ICU4XGraphemeClusterBreakIteratorUtf8;

  /**

   * Segments a UTF-16 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_utf16 Rust documentation} for more information.
   */
  segment_utf16(input: Uint16Array): ICU4XGraphemeClusterBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_latin1 Rust documentation} for more information.
   */
  segment_latin1(input: Uint8Array): ICU4XGraphemeClusterBreakIteratorLatin1;
}
