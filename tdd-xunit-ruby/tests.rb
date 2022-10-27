require_relative 'xunit'

class WasRun < TestCase
  def initialize(name)
    super
    @was_run = false
  end

  def was_run?
    @was_run
  end

  def test_method
    @was_run = true
  end
end

class TestCaseTest < TestCase
  def test_running
    test = WasRun.new('test_method')
    assert! !test.was_run?
    test.run
    assert! test.was_run?
  end
end

TestCaseTest.new('test_running').run
