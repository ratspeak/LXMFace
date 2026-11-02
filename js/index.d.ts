/**
 * Generate a deterministic identicon SVG for an LXMF/Reticulum address.
 *
 * Use the same case-sensitive ASCII address seed as Rust for matching output.
 * Addresses are not validated; arbitrary Unicode seeds can differ across languages.
 *
 * @param seed - Hash string (typically an LXMF hex address)
 * @param size - SVG width/height in pixels
 * @returns Self-contained SVG string
 */
export declare function lxmface(seed: string, size: number): string;
export default lxmface;
