class TestSuite
  def initialize
    @tests = []
  end

  def add(test)
    @tests << test
  end

  def run(result)
    @tests.each do |test|
      test.run result
    end
  end
end

class TestCase
  class AssertionFailed < StandardError; end

  def initialize(name)
    @name = name
  end

  def set_up; end

  def tear_down; end

  def run(result)
    result.test_started
    set_up
    begin
      send(@name)
    rescue StandardError
      result.test_failed
    end
    tear_down
  end

  def assert!(value)
    raise AssertionFailed, 'Assertion failed' unless value
  end
end

class TestResult
  def initialize
    @run_count = 0
    @error_count = 0
  end

  def test_started
    @run_count += 1
  end

  def test_failed
    @error_count += 1
  end

  def summary
    "#{@run_count} run, #{@error_count} failed"
  end
end
