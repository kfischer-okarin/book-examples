require_relative 'xunit'

class WasRun < TestCase
  attr_reader :log

  def was_run?
    @was_run
  end

  def set_up
    @was_run = false
    @log = 'set_up '
  end

  def test_method
    @was_run = true
    @log += 'test_method '
  end
end

class TestCaseTest < TestCase
  def test_template_method
    test = WasRun.new('test_method')
    test.run
    assert! test.log == 'set_up test_method '
  end
end

TestCaseTest.new('test_template_method').run
