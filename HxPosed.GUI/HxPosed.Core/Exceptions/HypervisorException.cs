namespace HxPosed.Core.Exceptions
{
    public class HypervisorException : Exception
    {
        private string _message, _source;
        public override string Message => _message;
        public override string Source => _source;

        internal HypervisorException(HypervisorError error)
        {
            _source = error.Source.ToString();
            _message = error.Error switch
            {
                ErrorCode.Ok => "Operation completed succesfully",
                ErrorCode.NotAllowed => "Missing permissions for operation",
                ErrorCode.Unknown => "Unknown error",
                ErrorCode.NotLoaded => "hxposed driver not loaded",
                _ => $"Unknown error: {(uint)error.Error:X}"
            };
        }
    }
}
