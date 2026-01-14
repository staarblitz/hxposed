using System;
using System.Buffers.Binary;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Cryptography
{
    // https://github.com/cocowalla/wyhash-dotnet/blob/master/src/WyHash/WyHash64.cs
    public class WyHash64 : HashAlgorithm
    {
        private ulong seed;
        private ulong length;

        public new static WyHash64 Create() => new WyHash64();
        public static WyHash64 Create(ulong seed) => new WyHash64(seed);

        private WyHash64()
        {
            this.HashSizeValue = sizeof(ulong) * 8;
        }

        private WyHash64(ulong seed = 0)
            : this()
        {
            this.seed = seed;
        }

        /// <summary>
        /// Convenience method to compute a WyHash hash and return the result as a 64-bit unsigned integer
        /// </summary>
        public static ulong ComputeHash64(byte[] array, ulong seed = 0)
        {
            seed = WyHashCore(array.AsSpan(), seed);
            return HashFinal(seed, (ulong)array.Length);
        }

        /// <summary>
        /// Convenience method to compute a WyHash hash and return the result as a 64-bit unsigned integer
        /// </summary>
        public static ulong ComputeHash64(ReadOnlySpan<byte> data, ulong seed = 0)
        {
            seed = WyHashCore(data, seed);
            return HashFinal(seed, (ulong)data.Length);
        }

        /// <summary>
        /// Convenience method to compute a WyHash hash and return the result as a 64-bit unsigned integer
        /// </summary>
        public static ulong ComputeHash64(ReadOnlySpan<char> str, ulong seed = 0)
        {
            var data = MemoryMarshal.Cast<char, byte>(str);
            seed = WyHashCore(data, seed);
            return HashFinal(seed, (ulong)data.Length);
        }

        /// <summary>
        /// Convenience method to compute a WyHash hash and return the result as a 64-bit unsigned integer
        /// </summary>
        public static ulong ComputeHash64(string str, ulong seed = 0)
        {
            var data = MemoryMarshal.Cast<char, byte>(str.AsSpan());
            seed = WyHashCore(data, seed);
            return HashFinal(seed, (ulong)data.Length);
        }

        public override void Initialize()
        {
            this.seed = 0;
            this.length = 0;
        }

        /// <inheritdoc />
        protected override void HashCore(byte[] array, int ibStart, int cbSize)
        {
            var len = cbSize - ibStart;
            this.length += (ulong)len;

            this.seed = WyHashCore(array.AsSpan(ibStart, cbSize), this.seed);
        }

#if NETCOREAPP3_0_OR_GREATER
        /// <inheritdoc />
        protected override void HashCore(ReadOnlySpan<byte> source)
        {
            this.length += (ulong)source.Length;
            this.seed = WyHashCore(source, this.seed);
        }

        /// <inheritdoc />
        public new bool TryComputeHash(ReadOnlySpan<byte> source, Span<byte> destination, out int bytesWritten)
        {
            HashCore(source);
            return TryHashFinal(destination, out bytesWritten);
        }

        /// <inheritdoc />
        protected override bool TryHashFinal(Span<byte> destination, out int bytesWritten)
        {
            var result = HashFinal(this.seed, this.length);

            if (BinaryPrimitives.TryWriteUInt64LittleEndian(destination, result))
            {
                bytesWritten = sizeof(ulong);
                return true;
            }

            bytesWritten = 0;
            return false;
        }
#endif

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static unsafe ulong WyHashCore(ReadOnlySpan<byte> span, ulong seed)
        {
            fixed (byte* pData = span)
            {
                byte* ptr = pData;

                var len = span.Length;
                var p = 0;

                for (int i = 0; i + 32 <= len; i += 32, p += 32)
                {
                    // Storing these in temp variables is slightly more performant (presumably it gives some kind of hint to the jitter)
                    var m1x = WyCore.Read64(ptr, p) ^ WyCore.Prime1;
                    var m1y = WyCore.Read64(ptr, p + 8) ^ WyCore.Prime2;
                    var m2x = WyCore.Read64(ptr, p + 16) ^ WyCore.Prime3;
                    var m2y = WyCore.Read64(ptr, p + 24) ^ WyCore.Prime4;

                    seed = WyCore.Mum(seed ^ WyCore.Prime0, WyCore.Mum(m1x, m1y) ^ WyCore.Mum(m2x, m2y));
                }

                seed ^= WyCore.Prime0;

                // After the loop we have between 1 and 31 bytes left to process
                switch (len & 31)
                {
                    case 1:
                        seed = WyCore.Mum(seed, WyCore.Read8(ptr, p) ^ WyCore.Prime1);
                        break;
                    case 2:
                        seed = WyCore.Mum(seed, WyCore.Read16(ptr, p) ^ WyCore.Prime1);
                        break;
                    case 3:
                        seed = WyCore.Mum(seed, ((WyCore.Read16(ptr, p) << 8) | WyCore.Read8(ptr, p + 2)) ^ WyCore.Prime1);
                        break;
                    case 4:
                        seed = WyCore.Mum(seed, WyCore.Read32(ptr, p) ^ WyCore.Prime1);
                        break;
                    case 5:
                        seed = WyCore.Mum(seed, ((WyCore.Read32(ptr, p) << 8) | WyCore.Read8(ptr, p + 4)) ^ WyCore.Prime1);
                        break;
                    case 6:
                        seed = WyCore.Mum(seed, ((WyCore.Read32(ptr, p) << 16) | WyCore.Read16(ptr, p + 4)) ^ WyCore.Prime1);
                        break;
                    case 7:
                        seed = WyCore.Mum(seed, ((WyCore.Read32(ptr, p) << 24) | (WyCore.Read16(ptr, p + 4) << 8) | WyCore.Read8(ptr, p + 6)) ^ WyCore.Prime1);
                        break;
                    case 8:
                        seed = WyCore.Mum(seed, WyCore.Read64Swapped(ptr, p) ^ WyCore.Prime1);
                        break;
                    case 9:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read8(ptr, p + 8) ^ WyCore.Prime2);
                        break;
                    case 10:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read16(ptr, p + 8) ^ WyCore.Prime2);
                        break;
                    case 11:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, ((WyCore.Read16(ptr, p + 8) << 8) | WyCore.Read8(ptr, p + 10)) ^ WyCore.Prime2);
                        break;
                    case 12:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read32(ptr, p + 8) ^ WyCore.Prime2);
                        break;
                    case 13:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, ((WyCore.Read32(ptr, p + 8) << 8) | WyCore.Read8(ptr, p + 12)) ^ WyCore.Prime2);
                        break;
                    case 14:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, ((WyCore.Read32(ptr, p + 8) << 16) | WyCore.Read16(ptr, p + 12)) ^ WyCore.Prime2);
                        break;
                    case 15:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, ((WyCore.Read32(ptr, p + 8) << 24) | (WyCore.Read16(ptr, p + 12) << 8) | WyCore.Read8(ptr, p + 14)) ^ WyCore.Prime2);
                        break;
                    case 16:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2);
                        break;
                    case 17:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, WyCore.Read8(ptr, p + 16) ^ WyCore.Prime3);
                        break;
                    case 18:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, WyCore.Read16(ptr, p + 16) ^ WyCore.Prime3);
                        break;
                    case 19:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, ((WyCore.Read16(ptr, p + 16) << 8) | WyCore.Read8(ptr, p + 18)) ^ WyCore.Prime3);
                        break;
                    case 20:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, WyCore.Read32(ptr, p + 16) ^ WyCore.Prime3);
                        break;
                    case 21:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, ((WyCore.Read32(ptr, p + 16) << 8) | WyCore.Read8(ptr, p + 20)) ^ WyCore.Prime3);
                        break;
                    case 22:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, ((WyCore.Read32(ptr, p + 16) << 16) | WyCore.Read16(ptr, p + 20)) ^ WyCore.Prime3);
                        break;
                    case 23:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, ((WyCore.Read32(ptr, p + 16) << 24) | (WyCore.Read16(ptr, p + 20) << 8) | WyCore.Read8(ptr, p + 22)) ^ WyCore.Prime3);
                        break;
                    case 24:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(seed, WyCore.Read64Swapped(ptr, p + 16) ^ WyCore.Prime3);
                        break;
                    case 25:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, WyCore.Read8(ptr, p + 24) ^ WyCore.Prime4);
                        break;
                    case 26:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, WyCore.Read16(ptr, p + 24) ^ WyCore.Prime4);
                        break;
                    case 27:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, ((WyCore.Read16(ptr, p + 24) << 8) | WyCore.Read8(ptr, p + 26)) ^ WyCore.Prime4);
                        break;
                    case 28:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, WyCore.Read32(ptr, p + 24) ^ WyCore.Prime4);
                        break;
                    case 29:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, ((WyCore.Read32(ptr, p + 24) << 8) | WyCore.Read8(ptr, p + 28)) ^ WyCore.Prime4);
                        break;
                    case 30:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                               WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, ((WyCore.Read32(ptr, p + 24) << 16) | WyCore.Read16(ptr, p + 28)) ^ WyCore.Prime4);
                        break;
                    case 31:
                        seed = WyCore.Mum(WyCore.Read64Swapped(ptr, p) ^ seed, WyCore.Read64Swapped(ptr, p + 8) ^ WyCore.Prime2) ^
                                    WyCore.Mum(WyCore.Read64Swapped(ptr, p + 16) ^ seed, ((WyCore.Read32(ptr, p + 24) << 24) | (WyCore.Read16(ptr, p + 28) << 8) | WyCore.Read8(ptr, p + 30)) ^ WyCore.Prime4);
                        break;
                }
            }

            return seed;
        }

        /// <inheritdoc />
        protected override byte[] HashFinal()
        {
            var result = HashFinal(this.seed, this.length);
            var r = BitConverter.GetBytes(result);
            return r;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static ulong HashFinal(ulong seed, ulong length) =>
            WyCore.Mum(seed, length ^ WyCore.Prime5);
    }

    internal static class WyCore
    {
        internal const ulong Prime0 = 0xa0761d6478bd642f;
        internal const ulong Prime1 = 0xe7037ed1a0b428db;
        internal const ulong Prime2 = 0x8ebc6af09c88c6e3;
        internal const ulong Prime3 = 0x589965cc75374cc3;
        internal const ulong Prime4 = 0x1d8e4e27c47d124f;
        internal const ulong Prime5 = 0xeb44accab455d165;

        /// <summary>
        /// Perform a MUM (MUltiply and Mix) operation. Multiplies 2 unsigned 64-bit integers, then combines the
        /// hi and lo bits of the resulting 128-bit integer using XOR
        /// </summary>
        /// <param name="x">First 64-bit integer</param>
        /// <param name="y">Second 64-bit integer</param>
        /// <returns>Result of the MUM (MUltiply and Mix) operation</returns>
        internal static ulong Mum(ulong x, ulong y)
        {
            var (hi, lo) = Multiply64(x, y);
            return hi ^ lo;
        }

        /// <summary>
        /// Multiplies 2 unsigned 64-bit integers, returning the result in 2 ulongs representing the hi and lo bits
        /// of the resulting 128-bit integer
        ///
        /// Source: https://stackoverflow.com/a/51587262/25758, but with a faster lo calculation
        /// </summary>
        /// <remarks>
        /// <seealso cref="System.Numerics.BigInteger"/> can perform multiplication on large integers, but it's
        /// comparatively slow, and an equivalent method allocates around 360B/call
        /// </remarks>
        /// <param name="x">First 64-bit integer</param>
        /// <param name="y">Second 64-bit integer</param>
        /// <returns>Product of <paramref name="x"/> and <paramref name="y"/></returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        [SuppressMessage("ReSharper", "JoinDeclarationAndInitializer")]
        internal static unsafe (ulong Hi, ulong Lo) Multiply64(ulong x, ulong y)
        {
            ulong hi;
            ulong lo;

            // Use BMI2 intrinsics where available
#if NETCOREAPP3_0_OR_GREATER
            if (System.Runtime.Intrinsics.X86.Bmi2.X64.IsSupported)
            {
                hi = System.Runtime.Intrinsics.X86.Bmi2.X64.MultiplyNoFlags(x, y, &lo);
                return (hi, lo);
            }
#endif

            lo = x * y;

            ulong x0 = (uint)x;
            ulong x1 = x >> 32;

            ulong y0 = (uint)y;
            ulong y1 = y >> 32;

            ulong p11 = x1 * y1;
            ulong p01 = x0 * y1;
            ulong p10 = x1 * y0;
            ulong p00 = x0 * y0;

            // 64-bit product + two 32-bit values
            ulong middle = p10 + (p00 >> 32) + (uint)p01;

            // 64-bit product + two 32-bit values
            hi = p11 + (middle >> 32) + (p01 >> 32);

            return (hi, lo);
        }

        /// <summary>
        /// Reads an unsigned 64-bit integer from a byte array.
        /// The value is constructed by combining the hi and lo bits of 2 unsigned 32-bit integers
        /// </summary>
        /// <param name="ptr">Pointer to a byte array from which to read the 64-bit integer from</param>
        /// <param name="start">Position from which to read the 64-bit integer</param>
        /// <returns>64-bit integer read from the byte array</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static unsafe ulong Read64Swapped(byte* ptr, int start)
        {
            var left = (ulong)*(uint*)(ptr + start);
            var right = (ulong)*(uint*)(ptr + start + 4);

            return left << 32 | right;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static unsafe ulong Read64(byte* ptr, int start) =>
            *(ulong*)(ptr + start);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static unsafe ulong Read32(byte* ptr, int start) =>
            *(uint*)(ptr + start);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static unsafe ulong Read16(byte* ptr, int start) =>
            *(ushort*)(ptr + start);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static unsafe ulong Read8(byte* ptr, int index) =>
            *(ptr + index);
    }
}
