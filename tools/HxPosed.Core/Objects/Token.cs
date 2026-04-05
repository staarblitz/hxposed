using HxPosed.Core.Types;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;

namespace HxPosed.Core.Objects
{
    public class Token : IObject
    {
        private Token() { }
        ~Token() => Dispose();
        private bool _disposed = false;

        public static Token FromRaw(HxToken token)
        {
            var me = new Token
            {
                Object = HxPosed.OpenObject(ServiceFunction.OpenToken, token)
            };

            me.AccountName = HxPosed.GetTokenAccountName(token);
            var source = 0UL;
            HxPosed.HxGetTokenSourceName(token, ref source).ThrowIfError();

            me.SourceName = Encoding.UTF8.GetString(BitConverter.GetBytes(source));

            return me;
        }

        public override string ToString() => $"{AccountName}@{SourceName}";

        public void Dispose()
        {
            if (_disposed) return;
            _disposed = true;
            GC.SuppressFinalize(this);

            if (Object != nint.Zero)
            {
                HxPosed.CloseObject(ServiceFunction.CloseToken, Object);
            }
        }

        public nint OpenHandle()
        {
            if (!Win32.OpenProcessToken(Win32.PROCESS_QUERY_INFORMATION, Win32.TOKEN_QUERY, out var token)) throw new Win32Exception();

            Handle.SwapAndUpgrade(token, Object);

            return token;
        }

        // should they really not throw exceptions?

        private TokenType GetTokenType()
        {
            var type = TokenType.Impersonation;
            HxPosed.HxGetTokenType(Object, ref type);
            return type;
        }

        private uint GetIntegrityLevelIndex()
        {
            var level = uint.MaxValue;
            HxPosed.HxGetTokenIntegrityLevelIndex(Object, ref level).ThrowIfError();
            return level;
        }

        private void SetIntegrityLevelIndex(uint level)
        {
            HxPosed.HxSetTokenIntegrityLevelIndex(Object, ref level);
        }

        private TokenImpersonationLevel GetTokenImpersonationLevel()
        {
            var level = TokenImpersonationLevel.Anonymous;
            HxPosed.HxGetTokenImpersonationLevel(Object, ref level).ThrowIfError();
            return level;
        }

        private void SetTokenImpersonationLevel(TokenImpersonationLevel level)
        {
            HxPosed.HxSetTokenImpersonationLevel(Object, ref level).ThrowIfError();
        }

        private TokenPrivileges GetTokenPresentPrivileges()
        {
            var privileges = new TokenPrivileges();
            HxPosed.HxGetTokenPresentPrivileges(Object, ref privileges).ThrowIfError();
            return privileges;
        }

        private void SetTokenPresentPrivileges(TokenPrivileges privileges)
        {
            HxPosed.HxSetTokenPresentPrivileges(Object, ref privileges).ThrowIfError();
        }

        private TokenPrivileges GetTokenEnabledPrivileges()
        {
            var privileges = new TokenPrivileges();
            HxPosed.HxGetTokenEnabledPrivileges(Object, ref privileges).ThrowIfError();
            return privileges;
        }

        private void SetTokenEnabledPrivileges(TokenPrivileges privileges)
        {
            HxPosed.HxSetTokenEnabledPrivileges(Object, ref privileges).ThrowIfError();
        }

        private TokenPrivileges GetTokenEnabledByDefaultPrivileges()
        {
            var privileges = new TokenPrivileges();
            HxPosed.HxGetTokenEnabledByDefaultPrivileges(Object, ref privileges).ThrowIfError();
            return privileges;
        }

        private void SetTokenEnabledByDefaultPrivileges(TokenPrivileges privileges)
        {
            HxPosed.HxSetTokenEnabledByDefaultPrivileges(Object, ref privileges).ThrowIfError();
        }

        public uint IntegrityLevelIndex
        {
            get => GetIntegrityLevelIndex();
            set => SetIntegrityLevelIndex(value);
        }

        public TokenType TokenType
        {
            get => GetTokenType();
        }

        public TokenImpersonationLevel ImpersonationLevel
        {
            get => GetTokenImpersonationLevel();
            set => SetTokenImpersonationLevel(value);
        }

        public TokenPrivileges PresentPrivileges
        {
            get => GetTokenPresentPrivileges();
            set => SetTokenPresentPrivileges(value);
        }

        public TokenPrivileges EnabledPrivileges
        {
            get => GetTokenEnabledPrivileges();
            set => SetTokenEnabledPrivileges(value);
        }

        public TokenPrivileges EnabledByDefaultPrivileges
        {
            get => GetTokenEnabledByDefaultPrivileges();
            set => SetTokenEnabledByDefaultPrivileges(value);
        }

        public string AccountName { get; private set; }
        public string SourceName { get; private set; }
        public HxToken Object { get; private set; }
    }
}
