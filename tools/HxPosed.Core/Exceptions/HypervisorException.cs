namespace HxPosed.Core.Exceptions
{
    public class HypervisorException : Exception
    {
        private string _message, _source;
        public override string Message => _message;
        public override string Source => _source;

        /// <summary>
        /// Exception generated from Rust's HypervisorError struct.
        /// </summary>
        /// <param name="error">Error struct returned from P/Invoke to hxposed_core</param>
        internal HypervisorException(HypervisorError error)
        {
            _source = error.Source.ToString();
            _message = (ErrorCode)error.Error switch
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
