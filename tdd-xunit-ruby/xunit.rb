class TestCase
  class AssertionFailed < StandardError; end

  def initialize(name)
    @name = name
  end

  def set_up; end

  def tear_down; end

  def run
    result = TestResult.new
    result.test_started
    set_up
    send(@name)
    tear_down
    result
  end

  def assert!(value)
    raise AssertionFailed, 'Assertion failed' unless value
  end
end

class TestResult
  def initialize
    @run_count = 0
  end

  def test_started
    @run_count += 1
  end

  def summary
    "#{@run_count} run, 0 failed"
  end
end
