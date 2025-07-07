---
name: Performance Issue
about: Report performance problems or slow network testing
title: '[PERFORMANCE] '
labels: ['performance', 'needs-triage']
assignees: ''

---

## Performance Issue Summary
A clear and concise description of the performance problem.

## Performance Context
**Which operation is slow?**
- [ ] DNS resolution testing
- [ ] Download speed measurement
- [ ] Docker registry testing
- [ ] Application startup
- [ ] UI responsiveness
- [ ] Test result processing
- [ ] Other (please specify)

## Timing Information
- **Expected duration:** [e.g., 5 seconds]
- **Actual duration:** [e.g., 30 seconds]
- **Number of concurrent tests:** [e.g., 26 DNS servers]
- **Test timeout setting:** [e.g., 10 seconds]

## Test Configuration
**Test Details:**
- Domain/URL being tested: [e.g., google.com]
- File size (for download tests): [e.g., 100MB]
- Number of DNS servers tested: [e.g., 26]
- Docker image (for registry tests): [e.g., ubuntu:latest]

## System Performance
**During the slow operation:**
- CPU usage: [e.g., 80%]
- Memory usage: [e.g., 2GB]
- Network utilization: [e.g., 10 Mbps]
- Disk I/O: [e.g., minimal]

## Network Environment
- **Internet connection speed:** [e.g., 100 Mbps down, 20 Mbps up]
- **Network type:** [e.g., WiFi, Ethernet, Mobile]
- **VPN/Proxy:** [e.g., Using VPN, Direct connection]
- **Firewall/Restrictions:** [e.g., Corporate firewall, No restrictions]

## Environment Information
**Operating System:** [e.g., Windows 11, macOS 14, Ubuntu 22.04]
**Bargozin Version:** [e.g., 0.1.0]
**Hardware:** [e.g., Intel i7, 16GB RAM, SSD]

## Steps to Reproduce
1. Go to '...'
2. Configure test with '...'
3. Start test and observe timing
4. See performance issue

## Expected vs Actual Performance
**Expected:** [e.g., All 26 DNS servers tested in 10 seconds]
**Actual:** [e.g., Tests taking 2+ minutes to complete]

## Comparison Data
Have you compared performance with:
- [ ] Different DNS servers
- [ ] Different test URLs/domains
- [ ] Different network conditions
- [ ] Different system resources
- [ ] Previous versions of Bargozin

## Logs/Profiling Data
If available, include any performance logs or profiling information:
```
[Paste performance data here]
```

## Workarounds
Any workarounds you've found to improve performance:

## Additional Context
Add any other context about the performance issue here.

## Checklist
- [ ] I have verified this is consistently reproducible
- [ ] I have tested with minimal configuration
- [ ] I have checked system resources during the issue
- [ ] I have tried different network conditions
- [ ] I have searched for similar performance issues 